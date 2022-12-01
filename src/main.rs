#[macro_use]
mod mymacros;
#[derive(Debug, Default)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            id: self.id,
            name: self.name.clone() + "被克隆",
            age: self.age,
        }
    }
}

fn main() {
    func!(php);
    php();

    let a = User {
        id: 123,
        ..Default::default()
    };
    let b = a.clone();
    println!("a: {:?}, b: {:?}", a, b);
}
