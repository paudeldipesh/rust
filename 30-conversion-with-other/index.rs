use std::fmt;
use std::str::FromStr;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    // conversion with other
    // convert any type to String
    let origin: Point = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");
    println!("{}", origin);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str: i32 = i32::from_str("20").unwrap();
    let sum: i32 = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}
