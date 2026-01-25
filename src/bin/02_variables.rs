fn main() {
    // 使用 let 声明变量
    // Rust 中变量默认是不可变的（immutable）
    let x = 5;
    println!("x = {}", x);

    // 使用 mut 关键字声明可变变量
    let mut y = 10;
    println!("y = {}", y);

    // 修改可变变量是允许的
    y = 20;
    println!("y = {}", y);
}
