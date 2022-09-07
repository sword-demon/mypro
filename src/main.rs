// mod lib; // lib.rs 或者 lib/mod.rs

fn get_user(uid:i32)->&'static str {
    let ret = if uid == 1 {
        "wujie"
    } else if uid == 2 {
        "张三"
    } else {
        "unknown"
    };
    ret
}

fn add (i:i32)->i32 {
    i+1
}

fn filter(html:&str) {
   for i in 1..=10  {
       println!("{}", i);
   }
}
fn main() {
    println!("user is {}", get_user(3));

    let a = 1; // 语句
    let b = if a == 1 {5}else {10};
    println!("{}", b);

    println!("{}", add(3));

    let html = "abcd";
    filter(html);
}
