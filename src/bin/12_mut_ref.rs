fn append_text(s: &mut String) {
    // 通过可变引用修改数据
    s.push_str(" language");
}

fn main() {
    let mut s = String::from("Rust");

    // 同一时间只能有一个可变引用
    append_text(&mut s);

    println!("{}", s);
}
