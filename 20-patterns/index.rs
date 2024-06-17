// patterns
#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    match_number(0);

    let p = Point { x: 3, y: 10 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point {
            x: 0..=5,
            y: y @ (10 | 20 | 30),
        } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({} , {})", x, y),
    }

    let msg: Message = Message::Hello { id: 10 };
    match msg {
        Message::Hello { id: id @ 3..=7 } => println!("Found and range in [3,7]: {}", id),
        Message::Hello {
            id: new_id @ (10 | 11 | 12),
        } => {
            println!("Found and range in [10,12]: {}", new_id)
        }
        Message::Hello { id } => println!("Found a single id: {}", id),
    }

    let num: Option<i32> = Some(4);
    let split: i32 = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    let numbers = (2, 4, 6, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    let mut v: String = String::from("hello");
    let r: &mut String = &mut v;
    match r {
        value => value.push_str(" world!"),
    }

    println!("Success!");
}

fn match_number(n: i32) {
    match n {
        1 => println!("one"),
        2 | 3 | 4 | 5 => println!("two to five"),
        6..=10 => println!("six to ten"),
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}