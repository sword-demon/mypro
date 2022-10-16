use models::user_model::{self, UserInfo};

// mod lib; // lib.rs 或者 lib/mod.rs
mod models;

fn set_user(u: &mut models::user_model::UserInfo) {
    u.user_id = 101;
    u.user_name = String::from("wujie");
    u.user_age = 10;
    u.user_tags = ["java", "php", "js", "go", "rust"];
}

fn main() {
    let mut user = models::user_model::new_user_info();
    set_user(&mut user);
    println!("{:?}", user);
}
