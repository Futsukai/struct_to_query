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




fn main() {
    let mut object = HelloStruct {
        age: None,
        money: 0,
        bank: None,
        name: "hello".to_owned(),
    };
    object.age = Some(32);
    println!("{}", object.get_http_query());
}
