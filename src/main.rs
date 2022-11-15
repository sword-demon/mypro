mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;

fn main() {
    let mut tags = Vec::new();
    tags.push(1);
    tags.push(2);
    // 左闭右开区间
    for i in &mut tags {
        // 解引用
        *i = *i + 10
    }
    println!("{:?}", tags);
}
