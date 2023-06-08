#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(thing: &Rectangle) -> u32 {
        thing.width * thing.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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
    let sq = Rectangle::square(10);
    println!("{:?}", sq);
    // println!(
    //     "can {:?} hold {:?}? {}.",
    //     rect1,
    //     rect2,
    //     rect1.can_hold(&rect2)
    // )
    // println!("area is {}", Rectangle::area(&rect1))
}
