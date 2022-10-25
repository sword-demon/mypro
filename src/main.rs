mod api;
mod models;
use api::Prods;
use api::Stock;
use models::book_model::*;

fn show_detail<T>(p: T)
where
    T: Prods + Stock,
{
    println!("商品的库存是: {}", p.get_stock());
}

fn main() {
    // 需要指明具体的类型
    let book1: Book = Prods::new(101, 25.6);
    show_detail(book1);
}
