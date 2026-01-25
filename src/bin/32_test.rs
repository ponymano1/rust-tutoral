fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

fn main() {
    println!("Run tests with: cargo test");
}
