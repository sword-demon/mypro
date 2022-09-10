// mod lib; // lib.rs 或者 lib/mod.rs

fn change(s: &mut String) {
   s.push_str("_19");
}

fn main() {
   let mut name = String::from("abc");
   change(&mut name);
   println!("{}", name);
}
