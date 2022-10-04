// mod lib; // lib.rs 或者 lib/mod.rs
mod models;

fn main() {
  let user = models::UserModel::new_user_model();
  println!("{:#?}", user);
}
