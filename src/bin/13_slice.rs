fn main() {
    let s = String::from("hello world");

    // slice 是对原数据的引用
    let hello = &s[0..5];

    println!("{}", hello);
}
