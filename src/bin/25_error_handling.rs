use std::fs::File;
use std::io::{self, Read};

// 演示 Result + ? 运算符
// ? 用来“向上传播错误”
// ? 表示一旦出现错误，就是return error
fn read_file() -> Result<String, io::Error> {
    // File::open 可能失败，返回 Result
    let mut file = File::open("test.txt")?;

    let mut content = String::new();

    // read_to_string 也可能失败
    file.read_to_string(&mut content)?;

    // 一切成功，返回 Ok
    Ok(content)
}

fn main() {
    match read_file() {
        Ok(text) => println!("file content:\n{}", text),
        Err(e) => println!("error happened: {}", e),
    }
}
