// Option 用来表示“有值 / 无值”
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn main() {
    match divide(10, 2) {
        Some(v) => println!("result = {}", v),
        None => println!("cannot divide"),
    }
}
