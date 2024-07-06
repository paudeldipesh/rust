#[allow(overflowing_literals)]
fn main() {
    // type conversion (coercion) - as
    let decimal: f32 = 97.123_f32;
    let integer: u8 = decimal as u8; // 97 -> a
    let _c1: char = decimal as u8 as char; // a
    let _c2: char = integer as char;
    assert_eq!(integer + 1, 'b' as u8); // 98

    assert_eq!(u8::MAX, 255);
    let v: u8 = 1000 as u8;
    println!("{}", v);

    assert_eq!(1000 as u16, 1000);
    assert_eq!(1000 as u8, 232);
    println!("1000 mod 256 is: {}", 1000 % 256); // u8::MAX + 1 = 255 + 1
    assert_eq!(-1_i8 as u8, 255);
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);
    unsafe {
        // 300.0 as u8 is 44 -> 300 - u8::MAX + 1 = 300 - 256 = 44
        println!("300.0 as u8 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156 -> u8::MAX + 1 = 256 - 100 = 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address: usize = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 points to the 2nd element in values
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b: *const [u8] = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13);
    }

    println!("Success!")
}
