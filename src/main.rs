mod lib; // lib.rs 或者 lib/mod.rs
fn main() {
    println!("u8最大值: {}, 最小值{}", u8::max_value(), u8::min_value());
    lib::helper::show_me();

    lib::config::show_version();
}
