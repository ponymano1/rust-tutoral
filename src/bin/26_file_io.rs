use std::fs;
use std::io;

// 演示最常见的文件读写方式
fn main() -> Result<(), io::Error> {
    // 写文件
    // 如果文件不存在，会自动创建
    fs::write("hello.txt", "Hello Rust!")?;

    // 读文件
    let content = fs::read_to_string("hello.txt")?;

    println!("file content: {}", content);

    Ok(())
}
