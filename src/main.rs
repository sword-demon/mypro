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
   me.to_string();
   println!("{:#?}", me);

   let a = {
    let inner = 2;
    inner * inner
  };
  println!("a={}", a);

  let tags = ["java", "php", "js"];
  println!("{:#?}", tags);

  for i in 0..tags.len()  { // range >=0 <3
      println!("{:#?}", tags[i]);
  }

  for item in tags.iter() {
    println!("{:#?}", item);
  }

  let mut cats:[u8;10] = [0;10];
  for i in 0..cats.len() {
      cats[i] = (i+1) as u8;
  }
  println!("{:#?}", cats);

  let my:(&str, u8) = ("wujie", 19);
  println!("{:#?}", my.0);
}
