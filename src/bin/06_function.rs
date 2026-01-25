// 定义一个函数
// 参数必须指定类型
// -> 后面是返回值类型
fn add(a: i32, b: i32) -> i32 {
    // Rust 中，最后一个表达式作为返回值
    // 注意：没有分号
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("result = {}", result);
}
