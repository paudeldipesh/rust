fn main() {
  let x: i32 = 5;
  let y = x; // Type of y is i32
  println!("y is {}", y);
  let _z = 10; // By default z is i32
  println!("Success!");

  // Declare a variable 'v' of type u16 (unsigned 16-bit integer)
  // Assign the value 38, initially of type u8 (unsigned 8-bit integer)
  // Converted to u16 using type casting
  let u: u16 = 99_u8 as u16;
  println!("u is {}", u);
  println!("Success!");

  let v: i32 = 5;
  assert_eq!("i32".to_string(), type_of(&v));
  println!("Success!");

  assert_eq!(i8::MAX, 127);
  assert_eq!(u8::MAX, 255);
  println!("Success!");

  // Beyond the valid range for u8;
  let p: u16 = 251_u16 + 8;
  // Method checked_add returns an Option<i16>
  // If the addition is valid and unwrap returns the value
  let q: i16 = i16::checked_add(251, 8).unwrap();
  println!("{}, {}", p, q);

  let r = 1_024 + 0x_ff + 0o_77 + 0b_1111_0000; // 1024 + 255 + 63 + 240
  assert_eq!(r == 1582, true);
  println!("Success!");

  let i = 1_000.000_1; // By default i is f64
  let _j: f32 = 0.12;
  let _z: f64 = 0.01_f64;
  assert_eq!(type_of(&i), "f64".to_string());
  println!("Success!");

  let is_equal: bool = 0.1_f32 + 0.2 as f32 == 0.3_f32;
  assert_eq!(is_equal, true);
  let _default_float = 0.1; // 0.10000000149011611938
  println!("Success!");

  let mut sum: i32 = 0;
  for i in -3..2 {
      sum += i
  }
  assert_eq!(sum == -5, true);
  for i in 97..=122 {
      // ASCII values for 'a' to 'z' (inclusive)
      println!("{}", char::from_u32(i).unwrap());
  }
  println!("Success!")
}

fn type_of<T>(_: &T) -> String {
  format!("{}", std::any::type_name::<T>())
}