mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;
#[derive(Debug)]
enum Sex {
    Male(String, i32),
    Female(String, i32),
}
#[derive(Debug)]
struct User {
    id: i32,
    sex: Sex,
}
fn main() {
    println!("{:?}", Sex::Male(String::from("男"), 1));
    println!("{:?}", Sex::Female(String::from("女"), 0));

    let u = User {
        id: 101,
        sex: Sex::Female(String::from("女"), 0),
    };
    println!("{:?}", u)
}
