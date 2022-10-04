#[derive(Debug)]
pub struct UserModel {
	user_id: i32,
	user_name: String,
	user_age: u8,
	user_tags: [&'static str; 5]
}

pub fn new_user_model()->UserModel {
	UserModel{
		user_id:0,
		user_name: String::new(),
		user_age: 0,
		user_tags: ["";5]
	}
}
