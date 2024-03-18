//! Structure to HTTP GET Query Parameters
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned, Result};
/// STQuery created a function to quickly convert the struct into HTTP GET query parameters;
/// By setting `use to_query::STQuery;` and tagging `#[derive(STQuery)]` onto the structure,
/// a new function `get_query() -> String` is added to the structure,
/// facilitating the generation of the query parameter string.
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
                        if !query_string.is_empty() {
                            query_string.push_str("&");
                        }

                        let mut content = self.#ident.as_ref().unwrap().to_string();
                        if !content.contains('=') {
                            content = format!("{}={}", stringify!(#ident), content);
                        }
                        query_string.push_str(&content);
                    }
            });
        } else {
            query_pieces.push(quote! {
                    if !query_string.is_empty() {
                        query_string.push_str("&");
                    }
                    let str = format!("{}={}", stringify!(#ident),self.#ident);
                    query_string.push_str(&str);
            });
        }
    }

    let builder_ident = syn::Ident::new(&struct_ident.to_string(), st.span());
    let ret = quote!(
        impl #builder_ident {
            fn get_query(&self) -> String {
                let mut query_string = String::new();
                #(#query_pieces)*
                query_string.to_string()
            }
        }

        impl std::fmt::Display for #builder_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.get_query())
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
