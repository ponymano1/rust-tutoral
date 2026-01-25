fn main() {
    let n = 3;

    // if 在 Rust 中是表达式，可以返回值
    let kind = if n > 0 { "positive" } else { "negative" };

    println!("{}", kind);
}
