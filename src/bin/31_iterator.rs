fn main() {
    let v = vec![1, 2, 3];

    // 迭代器链式调用
    let sum: i32 = v.iter().map(|x| x * 2).sum();

    println!("{}", sum);
}
