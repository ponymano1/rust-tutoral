// use 关键字用于“引入路径”，减少重复书写完整路径
// 本质上只是起一个更短、更方便的名字

use std::collections::HashMap;

fn main() {
    // 现在可以直接使用 HashMap
    // 而不需要写 std::collections::HashMap
    let mut map = HashMap::new();

    map.insert("apple", 3);
    map.insert("banana", 5);

    // Debug 打印
    println!("{:?}", map);
}
