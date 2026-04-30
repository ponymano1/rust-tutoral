fn main() {
    // 1) 不捕获环境：看起来像匿名函数
    let add = |a: i32, b: i32| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // 2) 捕获外部变量（默认尽量用借用 &T）：这是 Fn
    let base = 10;
    let add_base = |x: i32| x + base;
    println!("add_base(5) = {}", add_base(5));
    println!("base still usable: {base}");

    // 3) 捕获可变外部变量（需要 &mut）：这是 FnMut
    let mut sum = 0;
    let mut push = |x: i32| sum += x;
    push(3);
    push(4);
    println!("sum = {sum}");

    // 4) move 强制把所有权搬进闭包：常见于线程/异步；可能变成 FnOnce
    let s = String::from("hello");
    let consume = move || {
        // 这里把捕获的 String 移动走（drop），所以闭包只能调用一次（FnOnce）
        drop(s);
    };
    consume();
}
