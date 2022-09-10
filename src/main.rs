// mod lib; // lib.rs 或者 lib/mod.rs

#[derive(Debug)]
struct User {
   name: String,
   age: u8,
}

impl User {
    fn version(&self) {
      println!("1.0");
    }
    fn to_string(&self)->String {
      format!("我的名字是: {}, 我的年龄是: {}", &self.name, &self.age)
    }
}

fn main() {
   let me = User{
       name: String::from("无解"),
       age: 19
   };
   me.version();
   println!("{:#?}", me);
}
