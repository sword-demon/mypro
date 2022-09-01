mod lib{
    pub fn show_me() {
        let my_name = "wujie";
        let my_age = 18;
        println!("我的名字是: {}, 年龄是: {}", my_name, my_age);
    }
}
fn main() {
    println!("u8最大值: {}, 最小值{}", u8::max_value(), u8::min_value());
    lib::show_me();
}
