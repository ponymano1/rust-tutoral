// 使用引用借用数据，而不是获取所有权
fn print_len(s: &String) {
    println!("length = {}", s.len());
}

fn main() {
    let s = String::from("rust");

    // 传递引用，不转移所有权
    print_len(&s);

    // s 仍然可用
    println!("{}", s);
}
