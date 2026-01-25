fn main() {
    // shadowing（变量遮蔽）
    // 使用 let 再次声明同名变量

    let x = 5;
    // 新的 x 会“遮蔽”旧的 x
    let x = x + 1;
    let x = x * 2;

    // 最终的 x = (5 + 1) * 2 = 12
    println!("x = {}", x);
}
