# to_query 
Structure to HTTP GET Query Parameters.

[![Crates.io](https://img.shields.io/crates/v/to_query)](https://crates.io/crates/to_query)
[![Documentation](https://docs.rs/to_query/badge.svg)](https://docs.rs/to_query)

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
    let object = HelloStruct {
        name: "hello".to_owned(),
        age: None,
        money: 0,
        bank: Some(Bank {
            bank_name: "CoolBank".to_owned(),
            number: None,
        }),
    };
    assert_eq!(object.get_query(), "name=hello&money=0&bank_name=CoolBank")
```