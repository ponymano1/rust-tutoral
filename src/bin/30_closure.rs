fn main() {
    // 闭包可以捕获外部变量
    let add = |a, b| a + b;
    println!("{}", add(2, 3));
}
