// 模块用于组织代码
mod math {
    // pub 表示对外可见
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{}", math::add(2, 3));
}
