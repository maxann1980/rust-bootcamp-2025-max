#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 75,
    };

    println!(
        "Area of rectangle with width: {} and height: {} is {}",
        rect.width,
        rect.height,
        rect.area()
    );

    println!("rect is: {rect:?}"); // debug information in one line
    println!("rect is: {rect:#?}"); // debug information on several lines

    dbg!(&rect); // prints to stderr
}
