// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Move {x:u32, y:u32},
    ChangeColor (u64, u64, u64),
    Echo(String),
    Quit
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
