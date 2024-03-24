//! Structure to HTTP_GET/SQL Query Parameters
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned, Result};
/// STQuery created some function to quickly convert the struct into HTTP_GET/SQL query parameters;
/// By setting `use to_query::STQuery;` and tagging `#[derive(STQuery)]` onto the structure,
/// a new function `get_http_query() -> String` and `get_sql_query() -> String` is push into the structure,
/// facilitating the generation of the query parameter string.
/// Obtain a list of strings in the format ['name=value'], for custom purposes.
#[proc_macro_derive(STQuery)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(st: &syn::DeriveInput) -> Result<TokenStream> {
    let struct_ident = &st.ident;
    let fields = get_fields_from_derive_input(st)?;
    let mut query_pieces = Vec::new();
    for idx in 0..fields.len() {
        let field = &fields[idx];
        let ident = &field.ident;
        if is_optional(&field.ty) {
            query_pieces.push(quote! {
                    if !self.#ident.is_none() {

                        let mut content = self.#ident.as_ref().unwrap().to_string();
                        if !content.contains('=') {
                            content = format!("{}={}", stringify!(#ident), content);
                        }
                        query_strings.push(content);
                    }
            });
        } else {
            query_pieces.push(quote! {

                    let str = format!("{}={}", stringify!(#ident),self.#ident);
                    query_strings.push(str);
            });
        }
    }

    let builder_ident = syn::Ident::new(&struct_ident.to_string(), st.span());
    let ret = quote!(
        impl #builder_ident {
            /// Obtain a list of strings in the format ['name=value'], for custom purposes.
           pub fn get_strings(&self) -> Vec<String> {
                let mut query_strings = Vec::new();
                #(#query_pieces)*
                query_strings
            }

            pub fn get_http_query(&self) -> String {
                self.get_strings().join("&")
            }

            pub fn get_sql_query(&self) -> String {
                self.get_strings().join(",")
            }
        }

        impl std::fmt::Display for #builder_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.get_http_query())
            }
        }
    );
    Ok(ret.into())
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;
fn get_fields_from_derive_input(d: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = d.data
    {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(
        d,
        "Must define on a Struct, not Enum".to_string(),
    ))
}

fn is_optional(ty: &syn::Type) -> bool {
    // 模式匹配 外层是匹配的类型 （内部是解开到path的值, ..是忽略其他字段)
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        if let Some(seg) = path.segments.last() {
            if seg.ident == "Option" {
                return true;
            }
        }
    }
    false
}
