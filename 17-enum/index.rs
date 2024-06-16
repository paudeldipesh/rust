use crate::List::*;
// Enum
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
enum Number {
    Zero, // 0
    One,  // 1
    Two,  // 2
}

#[allow(dead_code)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
#[allow(dead_code)]
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Option enum as defined by the standard library
// enum Option<T> {
//     Some(T),
//     None,
// }

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
    println!("Home IP: {:?}", home);

    assert_eq!(Number::Zero as u8, Number1::Zero as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    let _msg1: Message = Message::Move { x: 1, y: 2 };
    let _msg1: Message = Message::Write(String::from("hello, world!"));

    let msg3: Message = Message::Move { x: 7, y: 7 };
    if let Message::Move { x: a, y: b } = msg3 {
        assert_eq!(a, b);
    } else {
        panic!("Never let this run!")
    }

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 3, y: 7 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    /*
    plus_one() expects an argument of type Option, so we have to wrap an i32 inside Some().
    x gets matched and if there is a Some() value, it gets incremented by one.
    */
    let five: Option<i32> = Some(5);
    let _six: Option<i32> = plus_one(five); // Some(6)
    let _none: Option<i32> = plus_one(None);
    if let Some(n) = _six {
        println!("{}", n);
    } else {
        panic!("Never let this run!");
    }

    // Create empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("Linked list has length: {}", list.len());
    println!("{}", list.stringify());

    println!("Success!");
}

fn show_message(msg: Message) {
    println!("{:?}", msg)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}