use models::user_model::{new_user_info, new_user_score_a, new_user_score_b, UserInfo};

// mod lib; // lib.rs 或者 lib/mod.rs
mod models;

fn set_user(u: &mut UserInfo) {
    u.user_id = 101;
    u.user_name = String::from("wujie");
    u.user_age = 10;
    u.user_tags = ["java", "php", "js", "go", "rust"];
}

fn main() {
    let mut user = new_user_info();
    set_user(&mut user);
    println!("{:?}", user);

    let mut user_score = new_user_score_a();
    user_score.user_id = 101;
    user_score.score = 10;
    println!("{:?}", user_score);
    println!("{:?}", user_score.get_user_type());
    println!("{:?}", user_score.get_user_id());

    let mut user_score_b = new_user_score_b();
    user_score_b.user_id = "#EFff";
    user_score_b.score = 10.0;
    println!("{:?}", user_score_b);
    println!("{:?}", user_score_b.get_user_type());
    println!("{:?}", user_score_b.get_user_id());
}
