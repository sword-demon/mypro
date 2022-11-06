mod api;
mod models;
// use api::Prods;
// use api::Stock;
// use models::book_model::*;

fn show_name<'a>(name: &'a str) -> &'a str {
    name
}

fn main() {
    let name = "lisi";
    show_name(name);
}
