#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// we can create multiple implementation block (impl) for a struct
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: (size), height: (size) }
    }
}

fn main() {
    let rect: Rectangle = Rectangle{width: 30, height: 50};

    println!("rect: {:#?}", rect);

    println!("The area of the rectangle in {} sq units.", {rect.area()});

    let rect_square = Rectangle::square(20);

    println!("square: {:#?}", rect_square);
}

