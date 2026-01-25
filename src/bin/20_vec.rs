fn main() {
    // Vec 是动态数组
    let mut v = vec![1, 2, 3];

    v.push(4);

    for i in v {
        println!("{}", i);
    }
}
