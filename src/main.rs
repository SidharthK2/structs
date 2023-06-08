#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!(
        "can {:?} hold {:?}? {}.",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    )
}
