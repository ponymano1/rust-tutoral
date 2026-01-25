use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    for (name, score) in scores {
        println!("{}: {}", name, score);
    }
}
