// 定义结构体
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 20,
    };

    println!("{} is {}", user.name, user.age);
}
