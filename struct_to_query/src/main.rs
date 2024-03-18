use to_query::STQuery;

#[derive(STQuery)]
struct HelloStruct {
    name: String,
    age: Option<u32>,
    money:u32,
    bank: Option<Bank>,
}
#[derive(STQuery)]
struct Bank {
    bank_name: String,
    number:Option<u32>,
}

fn main() {
    let h = HelloStruct {
        name:"name".to_owned(),
        age: Some(32),
        money:0,
        bank: Some(Bank {
            bank_name:"bank".to_owned(),
            number:Some(10001),
        }),
    };
    let t = h.get_query();


    if t.is_empty() {
        println!("empty query");
    }


    println!("{}", t);
}
