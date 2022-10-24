// 写成接口或抽象类

// crate 比 mod 级别更高
use crate::models::book_model::Book;

pub trait Prods {
    fn get_price(&self) -> f32;
    fn new(id: i32, price: f32) -> Self;
}

// 使用 trait 对某一个 struct 定义方法
impl Prods for Book {
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }

    fn new(id: i32, price: f32) -> Book {
        Book { id, price }
    }
}
