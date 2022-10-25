mod api;
mod models;
use api::Prods;
use models::{book_model::*, phone_model::Phone};

fn sum1<T: Prods>(p1: T, p2: T) {
    println!("商品总价是: {}", p1.get_price() + p2.get_price())
}

fn sum2<T: Prods, U: Prods>(p1: T, p2: U) {
    println!("商品总价是: {}", p1.get_price() + p2.get_price())
}

fn main() {
    // 需要指明具体的类型
    let book1: Book = Prods::new(101, 25.6);
    let book2: Book = Prods::new(102, 30.6);
    sum1(book1, book2);
    let book3: Book = Prods::new(103, 10.3);
    let phone1: Phone = Prods::new(104, 1300.2);
    sum2(book3, phone1);
}
