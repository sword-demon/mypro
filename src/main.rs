mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;
#[derive(Debug)]
enum Sex {
    Male,
    Female,
}
#[derive(Debug)]
struct User {
    id: i32,
    sex: Sex,
}

fn check(u: User) {
    if let Sex::Male = u.sex {
        println!("男性");
    } else {
        println!("女性")
    }
}

fn main() {
    let u = User {
        id: 101,
        sex: Sex::Female,
    };
    println!("{:?}", u);
    check(u);
}
