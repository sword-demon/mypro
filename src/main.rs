mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;
#[derive(Debug)]
enum Sex {
    Male(String),
    Female(String),
}
#[derive(Debug)]
struct User {
    id: i32,
    sex: Option<String>,
}

fn check(u: User) {
    println!("{}", u.sex.unwrap());
    // if let Some(s) = u.sex {
    // println!("{}", s);
    // } else {
    // println!("女性")
    // }
}

fn main() {
    let u = User {
        id: 101,
        sex: Some(String::from("女")),
    };
    println!("{:?}", u);
    check(u);
}
