// mod lib; // lib.rs 或者 lib/mod.rs

fn main() {
   let first_name = String::from("无");
   let last_name = String::from("解");

//    let name = first_name + &last_name; // &str &String => &str 自动转化
//    println!("{}", name);

   let name1 = format!("{}{}", first_name, last_name);
   println!("{}", name1);
}
