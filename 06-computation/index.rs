use std::ops::{Range, RangeInclusive};

fn main() {
    for c in 'A'..='C' {
        println!("{}", c as u8);
    }

    assert_eq!(1..5, Range { start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));
    println!("Success!");

    // Computations
    assert_eq!(1u32 + 3 as u32 == 4 as u32, true);
    assert_eq!(1_i32 - 2 as i32 == -1 as i32, true);
    assert_eq!(1_i8 - 2 as i8 == -1 as i8, true);
    assert_eq!(3 * 50, 150); // Default i32
    assert_eq!(9.6_f32 / 3.2 as f32, 3.0 as f32);
    assert_eq!(24 % 5, 4);
    println!("Success!");

    // Short-circuiting boolean logic
    assert_eq!(true && false, false);
    assert_eq!(true || false, true);
    assert_eq!(!true == false, true);
    println!("Success!");

    // Bitwise operations
    println!("0011 AND 0101 = {:04b}", 0b0011_u32 & 0b0101 as u32);
    println!("0011 OR 0101 = {:04b}", 0b0011_u32 | 0b0101 as u32);
    println!("0011 XOR 0101 = {:04b}", 0b0011_u32 ^ 0b0101 as u32);
    println!("1 << 5 = {}", 1_u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80 as u32 >> 2); // Hexa - Binary - Hexa
    println!("Success!");
}