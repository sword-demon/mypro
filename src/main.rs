mod api;
mod models;
use api::Prods;
use api::Stock;
use models::book_model::*;

fn sum(p1: Book, p2: Book) {
    println!(
        "p1的价格是: {}, p2的价格是: {}, 商品总价是: {}",
        p1.get_price(),
        p2.get_price(),
        p1 + p2
    );
}

fn main() {
    // 需要指明具体的类型
    let book1: Book = Prods::new(101, 25.6);
    let book2: Book = Prods::new(110, 30.5);
    sum(book1, book2);
}
