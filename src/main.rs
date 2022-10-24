mod api;
mod models;
use api::Prods;
use models::book_model::*;

fn main() {
    // 需要指明具体的类型
    let book: Book = Prods::new(101, 25.0);
    println!("{:?}", book.get_price());
}
