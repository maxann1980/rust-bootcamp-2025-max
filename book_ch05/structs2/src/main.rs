// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    println!("Point : {},{},{}", point.0, point.1, point.2);
    println!("Color : {},{},{}", black.0, black.1, black.2);
}
