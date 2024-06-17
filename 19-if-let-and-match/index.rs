#![allow(dead_code)]
// if let and match
enum Direction {
    East,
    West,
    North,
    South,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MyEnum {
    Foo,
    Bar,
}

enum Foo {
    Bar(u8),
}

enum Hotel {
    Room,
    Food,
    Price(u32),
}

fn main() {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East!"),
        Direction::South | Direction::North => {
            println!("South or North!");
        }
        _ => println!("West!"),
    }

    let boolean: bool = true;
    let binary: u8 = match boolean {
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 10, y: 8 },
        Message::ChangeColor(0, 160, 255),
    ];
    for msg in msgs {
        show_message(msg);
    }

    let alphabets = ['a', '0', 'e', 'A', 'i', '9', 'o', 'u', 'Q'];
    for alpha in alphabets {
        assert!(matches!(alpha, 'a'..='u' | 'A'..='Q' | '0'..='9'));
    }

    let mut count = 0;
    let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    assert_eq!(count, 3);

    let opt: Option<i32> = Some(8);
    // match opt {
    //     Some(i) => {
    //         println!("This is a really long string and `{:?}`.", i);
    //         println!("Success!");
    //     }
    //     _ => {}
    // }
    if let Some(i) = opt {
        println!("This is a really long string.");
        println!("Value: {:?}", i);
    }

    let a: Foo = Foo::Bar(1);
    #[allow(irrefutable_let_patterns)]
    if let Foo::Bar(i) = a {
        println!("foobar holds: {}.", i);
    }

    let a: Hotel = Hotel::Price(800);
    // if let Hotel::Room = a {
    //     println!("Match: Hotel::Room");
    // } else if let Hotel::Food = a {
    //     println!("Match: Hotel::Food");
    // } else {
    //     println!("Match Others");
    // }
    match a {
        Hotel::Room => println!("Match: Hotel::Room"),
        Hotel::Food => println!("Match: Hotel::Food"),
        _ => println!("Match others"),
    }

    let age: Option<i32> = Some(30);
    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("Age value: {}", age),
        _ => (),
    }

    println!("Success!")
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            assert_eq!(a, 10);
            assert_eq!(b, 8);
        }
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 160);
            assert_eq!(b, 255);
        }
        _ => println!("No data in these variants!"),
    }
}