fn main() {
    {
        // s 只在这个作用域中有效
        let s = String::from("scope");
        println!("{}", s);
    }
    // 作用域结束，s 被自动 drop（释放内存）
}
