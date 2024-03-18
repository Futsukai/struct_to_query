use to_query::STQuery;

#[derive(STQuery)]
struct HelloStruct {
    name: String,
    age: Option<u32>,
    money:u32,
    bank: Option<Bank>,
}
#[derive(Debug,Clone,STQuery)]
struct Bank {
    name: String,
    number:Option<u32>,
}

fn main() {
    let h = HelloStruct {
        name:"".to_owned(),
        age: Some(32),
        money:0,
        bank: Some(Bank {
            name:"bank".to_owned(),
            number:None,
        }),
    };
    let t = h.get_query();


    if t.is_empty() {
        println!("empty query");
    }


    println!("{}", t);
}
