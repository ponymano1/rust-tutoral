use std::fs::File;

fn main() {
    // Result 用于错误处理
    let result = File::open("not_exist.txt");

    match result {
        Ok(_) => println!("file opened"),
        Err(e) => println!("error: {}", e),
    }
}
