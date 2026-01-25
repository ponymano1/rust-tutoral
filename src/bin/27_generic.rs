// 泛型函数
// T: PartialOrd 表示 T 实现了 PartialOrd 特征
// PartialOrd 是一个trait，表示可以比较大小
//pub trait PartialOrd<Rhs = Self> {
//    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
//}
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", max(3, 5));
}
