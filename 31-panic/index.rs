fn main() {
    // Panic
    // This will cause a panic because the beverage is "lemonade"
    drink("lemon");
    println!("This line will not print if there is a panic!");

    assert_eq!("abc".as_bytes(), [97, 98, 99]);
    let v: Vec<i32> = vec![1, 2, 3];
    let _ele: i32 = v[2];
    let _ele: &i32 = v.get(1).unwrap(); // Some(2) -> 2
    let v = production_rate_per_hour(2);
    println!("{}", v);
    divide(15, 2);

    println!("Success!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Match found!");
        panic!();
    } else {
        println!("Match not found!");
    }
}

fn production_rate_per_hour(speed: u16) -> f64 {
    let cph: u16 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u16) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}

fn divide(x: u8, y: u8) {
    println!("{}", (x as f32 / y as f32) as f32)
}
