// 写成接口或抽象类

// crate 比 mod 级别更高
use crate::models::book_model::Book;
use crate::models::phone_model::Phone;

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

impl Prods for Phone {
    fn new(id: i32, price: f32) -> Phone {
        Phone { id, price }
    }
    fn get_price(&self) -> f32 {
        &self.price + 20.0
    }
}

impl std::ops::Add<Book> for Book {
    // 定义输出类型是什么
    type Output = f32;
    // 实现加法 A, B B的类型为对应的book类型
    fn add(self, rhs: Book) -> f32 {
        self.get_price() + rhs.get_price()
    }
}
