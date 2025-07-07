#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
        rect_area(&rect)
    );

    println!("rect is: {rect:?}"); // debug information in one line
    println!("rect is: {rect:#?}"); // debug information on several lines

    dbg!(&rect); // prints to stderr
}

fn rect_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
