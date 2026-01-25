// 枚举定义一组可能的值
enum Direction {
    Up,
    Down,
}

fn main() {
    let d = Direction::Up;

    match d {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
    }
}
