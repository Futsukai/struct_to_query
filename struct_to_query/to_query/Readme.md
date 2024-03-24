# to_query 
Structure to HTTP_GET/SQL Query Parameters.

[![Crates.io](https://img.shields.io/crates/v/to_query)](https://crates.io/crates/to_query)
[![Documentation](https://docs.rs/to_query/badge.svg)](https://docs.rs/to_query)

> Multiple identical structs may result in duplicated output, but they can be freely combined by accessing arrays `get_strings()` to obtain the desired output.

```rust
    use to_query::STQuery;
    #[derive(STQuery)]
    struct HelloStruct {
        name: String,
        age: Option<u32>,
        money: u32,
        bank: Option<Bank>,
    }
    #[derive(STQuery)]
    struct Bank {
        bank_name: String,
        number: Option<u32>,
    }
    
    fn fill_struct_test() {
        let object = HelloStruct {
            name: "hello".to_owned(),
            age: Some(123),
            money: 0,
            bank: Some(Bank {
                bank_name: "CoolBank".to_owned(),
                number: Some(101),
            }),
            bank2: Bank{
                bank_name: "abc".to_string(),
                number: None,
            },
        };
        assert_eq!(
            object.get_http_query(),
            "name=hello&age=123&money=0&bank_name=CoolBank&number=101&bank_name=abc"
        );
        assert_eq!(
            object.get_sql_query(),
            "name=hello,age=123,money=0,bank_name=CoolBank,number=101,bank_name=abc"
        );

        assert_eq!(
            object.get_strings().get(2),
            Some("money=0".to_string()).as_ref()
        );

        assert_eq!(
            object.get_strings().last(),
            Some("bank_name=abc".to_string()).as_ref()
        );
    }
```