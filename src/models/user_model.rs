#[derive(Debug)]
pub struct UserInfo {
    pub user_id: i32,
    pub user_name: String,
    pub user_age: u8,
    pub user_tags: [&'static str; 5],
}

pub fn new_user_info() -> UserInfo {
    UserInfo {
        user_id: 0,
        user_name: String::new(),
        user_age: 0,
        user_tags: [""; 5],
    }
}

#[derive(Debug)]
pub struct UserScore<A, B> {
    // 泛型
    pub user_id: A,
    pub score: B,
    pub comment: &'static str,
}

impl<A, B> UserScore<A, B> {
    pub fn get_user_type(&self) -> &'static str {
        // 表达式
        &self.comment
    }

    pub fn get_user_id(&self) -> &A {
        &&self.user_id
    }
}

pub fn new_user_score_a() -> UserScore<i32, i32> {
    UserScore {
        user_id: 0,
        score: 0,
        comment: "A类用户",
    }
}

pub fn new_user_score_b() -> UserScore<&'static str, f32> {
    UserScore {
        user_id: "",
        score: 0.0,
        comment: "B类用户",
    }
}
