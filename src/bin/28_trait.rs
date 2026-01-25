// trait 类似接口
trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

fn main() {
    let d = Dog;
    d.speak();
}
