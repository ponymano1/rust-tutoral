fn main() {
    // Rust 迭代器（Iterator）核心：一个“按需产出元素”的状态机
    //
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }
    //
    // 你可以把它理解成：每次调用 next() 要一个元素，直到返回 None。

    // ===== 1) iter / iter_mut / into_iter：决定 Item 是“引用”还是“所有权” =====
    let v = vec![1, 2, 3];

    // v.iter() -> 迭代 &i32（只读借用，不移动 v 的所有权）
    let sum_doubled: i32 = v.iter().map(|x| x * 2).sum();
    // 这里 x 的类型是 &i32；之所以能写 x * 2，是因为整数运算会对引用做自动解引用（等价于 (*x) * 2）。
    println!("sum_doubled from iter(): {sum_doubled}");
    println!("v still usable after iter(): {:?}", v);

    // v.iter_mut() -> 迭代 &mut i32（可变借用，允许原地修改）
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().for_each(|x| *x *= 10);
    // for_each 是“消费器/终结操作”：它会把迭代器跑完
    println!("v2 after iter_mut().for_each: {:?}", v2);

    // v.into_iter() -> 迭代 i32（移动元素所有权；v3 之后不能再用）
    let v3 = vec![1, 2, 3];
    let collected: Vec<i32> = v3.into_iter().map(|x| x + 100).collect();
    // collect 也是终结操作：把迭代器“收集”成容器（Vec/HashMap/...），需要类型能推导出来
    println!("collected from into_iter(): {:?}", collected);
    // println!("{:?}", v3); // 取消注释会报错：v3 已被 into_iter() 消耗（move）

    // ===== 2) 惰性（lazy）：适配器不执行，直到遇到终结操作 =====
    let v4 = vec![1, 2, 3];

    // map/filter/take/enumerate/zip/... 都是“迭代器适配器”：
    // 它们返回新的迭代器，把“规则”串起来，但不会立刻跑（惰性）。
    //
    // inspect 也是适配器：常用来在流水线中“偷看”每一步的值，便于理解惰性执行与短路行为。
    let pipeline = v4.iter().inspect(|x| println!("inspect(source): {x}"));

    println!("pipeline built (nothing executed yet)");

    // 一旦调用终结操作（比如 collect/sum/fold/find/for_each/count/...）
    // 才会真正开始从上游 next() 拉取数据，逐个穿过适配器链。
    let out: Vec<i32> = pipeline
        .map(|x| {
            // x: &i32（来自 v4.iter()）
            println!("map called on {x}");
            x * 2 // 产出 i32
        })
        .filter(|x| {
            // 注意：filter 的闭包参数类型是 &Item，所以这里是 &i32（因为 map 之后 Item = i32）
            println!("filter called on {x}");
            *x % 3 != 0
        })
        .collect();

    println!("out after consuming pipeline: {:?}", out);

    // ===== 3) 常见“终结操作/消费器” =====
    let v5 = vec![1, 2, 3, 4, 5];

    // sum：求和（需要你能确定返回类型）
    let s: i32 = v5.iter().sum();
    println!("sum = {s}");

    // fold：自定义“归约”，比 sum 更通用（可以做乘积、拼接字符串、构造结构体等）
    let product: i32 = v5.iter().fold(1, |acc, x| acc * x);
    println!("product via fold = {product}");

    // find：找到第一个满足条件的元素就停止（短路）
    let first_even = v5.iter().find(|x| **x % 2 == 0);
    println!("first_even = {first_even:?}");

    // any / all：短路判断
    let has_big = v5.iter().any(|x| *x >= 5);
    let all_positive = v5.iter().all(|x| *x > 0);
    println!("any >= 5? {has_big}, all > 0? {all_positive}");

    // ===== 4) 常见“适配器”小抄 =====
    // enumerate：给元素配上索引（从 0 开始）
    for (idx, val) in ["a", "b", "c"].iter().enumerate() {
        println!("enumerate: idx={idx}, val={val}");
    }

    // take / skip：截取迭代器的一段（也都是惰性的）
    let first_two: Vec<i32> = (1..=10).take(2).collect();
    let skip_two: Vec<i32> = (1..=5).skip(2).collect();
    println!("take(2) from 1..=10 => {first_two:?}");
    println!("skip(2) from 1..=5  => {skip_two:?}");

    // zip：把两个迭代器“配对”到一起；长度取较短的那个
    let zipped: Vec<(i32, char)> = (1..=3).zip(['x', 'y', 'z']).collect();
    println!("zip => {zipped:?}");

    // ===== 5) 一个容易混淆点：Iterator 与 IntoIterator =====
    //
    // - Iterator：有 next() 的“迭代器本体”
    // - IntoIterator：可以“变成迭代器”的东西（比如 Vec、&Vec、&mut Vec、Range 等）
    //
    // for 循环用的是 IntoIterator（语义上）：它会先把右边变成迭代器再循环 next()
    let mut total = 0;
    for x in &v {
        // &v 走的是 v.iter()（不移动 v）
        total += *x;
    }
    println!("for x in &v => total={total}, v still = {:?}", v);
}
