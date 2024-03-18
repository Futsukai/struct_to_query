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

#[test]
fn single_struct_test() {
    let object = HelloStruct {
        name: "hello".to_owned(),
        age: None,
        money: 0,
        bank: None,
    };
    assert_eq!(object.get_query(), "name=hello&money=0")
}

#[test]
fn inner_struct_test() {
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
}


#[test]
fn fill_struct_test() {
    let object = HelloStruct {
        name: "hello".to_owned(),
        age: Some(123),
        money: 0,
        bank: Some(Bank {
            bank_name: "CoolBank".to_owned(),
            number: Some(101),
        }),
    };
    assert_eq!(object.get_query(), "name=hello&age=123&money=0&bank_name=CoolBank&number=101")
}