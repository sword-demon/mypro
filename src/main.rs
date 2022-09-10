// mod lib; // lib.rs 或者 lib/mod.rs

fn main() {
   let name = String::from("abc");
   let you = name.clone();
   println!("{:p}", name.as_ptr());
   println!("{:p}", you.as_ptr());
}
