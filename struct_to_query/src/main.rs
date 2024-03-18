use to_query::STQuery;

#[derive(STQuery)]
struct HelloStruct {
    name: String,
    age: Option<u32>,
    money:u32,
    // bank: Option<Bank>,
}
// #[derive(Debug,Clone,STQuery)]
// struct Bank {
//     name: String,
//     number:Option<u32>,
// }

fn main() {
    let h = HelloStruct {
        name:"".to_owned(),
        age: None,
        money:0,
    };
    let t = h.get_query();


    if t.is_empty() {
        println!("empty query");
    }


    println!("{}", t);
}
