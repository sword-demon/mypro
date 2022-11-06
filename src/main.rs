mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;

fn max<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 12;
    let b = 21;
    println!("最大值是: {}", max(&a, &b));
}
