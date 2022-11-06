mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;

#[derive(Debug)]
struct User<'a> {
    id: &'a i32,
}

fn main() {
    let mut id = 11;
    let u = User { id: &id };
    println!("{:?}", u);
    id = 107;
    println!("{:?}", id);
}
