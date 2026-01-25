fn main() {
    // String 是堆分配类型
    let s1 = String::from("hello");

    // s1 的所有权被移动到 s2
    let s2 = s1;

    // println!("{}", s1); // ❌ 编译错误：s1 已失效
    println!("{}", s2);
}
