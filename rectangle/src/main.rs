fn main() {
    println!("Hello, world!");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("rect is {}", rect);
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);

    let rect = Rectangle::square(5);
    println!("area of rect is {}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
