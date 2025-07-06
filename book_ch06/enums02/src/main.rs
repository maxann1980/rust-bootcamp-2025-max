#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Resize { width: u32, height: u32 },
    Move(Point),
    Echo(String),
    ChangeColor(u32, u32, u32),
    Quit,
    // TODO: Define the different variants used below.
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
        match message {
            Message::Resize { width, height } => {
                println!("{}, {}", width, height);
            }

            Message::Move(Point { x, y }) => {
                println!("Move to ({x}, {y})");
            }

            Message::Echo(str1) => {
                println!("{str1}");
            }

            Message::ChangeColor(x, y, z) => {
                println!("{x}, {y}, {z}");
            }

            Message::Quit => {
                println!("quit");
            }
        }
    }
}
