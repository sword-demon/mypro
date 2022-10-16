// 图书实体

#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub price: f32,
}

// 所谓的实例化函数
pub fn new_book(id: i32, price: f32) -> Book {
    // 表达式
    Book {
        id: id,
        price: price,
    }
}
