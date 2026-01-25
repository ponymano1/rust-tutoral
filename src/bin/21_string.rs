fn main() {
    // String 是堆分配、可变字符串
    let s1 = String::from("hello");

    // &str 是字符串切片，通常是不可变的
    let s2 = "world";

    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);
}
