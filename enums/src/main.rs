enum IpAddrKind {
    V4,
    V6,
}

// Enums can hold DATA directly (more powerful than most languages' enums!)
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enums can hold very different types per variant, including structs
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named fields, like an anonymous struct
    Write(String),              // one String
    ChangeColor(i32, i32, i32), // three i32s
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Write: {text}"),
            Message::ChangeColor(r, g, b) => println!("Color: ({r}, {g}, {b})"),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // You MUST handle the None case explicitly before using the value — no null pointer exceptions!
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;         // ❌ error: cannot add i8 and Option<i8> directly

    match y {
        Some(val) => println!("Got {val}"),
        None => println!("Nothing here"),
    }

    // Common helper methods on Option (very frequently used!)
    let val = y.unwrap_or(0); // default if None
    let doubled = y.map(|v| v * 2); // transform if Some
    let is_some = y.is_some();
    println!("{val} {doubled:?} {is_some}");
}
