// enums give you a way of saying a value is one of a possible set of values. 
// For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle. 
// To do this, Rust allows us to encode these possibilities as an enum.

// enum IpAddrKind {
//     V4,
//     V6,
// }


// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct variation

// enums can use implementaion methods


impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();


// Rust has no Null value, as it has led to error in other programming languages
// Instead... we have the Option Enum

let some_number = Some(5);
let some_char = Some('e');

// need to qualify what Some might have if we want to use None
let absent_number: Option<i32> = None;


let x: i8 = 5;
let y: Option<i8> = Some(5);
//this throws an error because we cannot add Option<i8> and i8
let sum = x + y;
