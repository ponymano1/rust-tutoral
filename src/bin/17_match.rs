fn main() {
    let n = 2;

    // match 必须覆盖所有情况
    match n {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
}
