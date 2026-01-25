struct User {
    name: String,
}

impl User {
    // &self 表示借用当前实例
    fn say_hi(&self) {
        println!("Hi, {}", self.name);
    }
}

fn main() {
    let u = User { name: "Bob".into() };

    u.say_hi();
}
