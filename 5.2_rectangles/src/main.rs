#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods for Rectangle Struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }

    // Associated Function (Constructor)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let s = Rectangle::square(3);

    println!("rect is {:#?}", rect); // pretty-print specifier
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    println!("s is {:#?}", s);
}

