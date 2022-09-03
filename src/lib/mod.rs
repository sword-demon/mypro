pub mod config; // config.rs 或者 config/mod.rs
pub mod helper {
	pub fn show_me() {
		let my_name = "wujie";
		let my_age = 18;
		println!("我的名字是: {}, 年龄是: {}", my_name, my_age);
	}
}
