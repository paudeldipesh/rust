fn main() {
  // String
  let mut s: String = String::from("Hello ");
  s.push_str("World");
  s.push('!');
  move_ownership(s.clone());
  assert_eq!("Hello World!", s);

  let s: String = String::from("Dipesh Paudel");
  let slice1: &str = &s; // &String -> &str (as_str())
  assert_eq!(slice1, "Dipesh Paudel");
  let slice2: &str = &s[..6];
  assert_eq!(slice2, "Dipesh");
  let mut slice3: String = s;
  slice3.push('!');
  assert_eq!(slice3, "Dipesh Paudel!");

  let s: String = String::from("Hello World"); // 1
  let slice: &str = &s;
  let s: String = slice.to_string(); // 2
  assert_eq!(s, "Hello World");

  let s: String = String::from("Hello World");
  let slice1: &str = &s[..1];
  assert_eq!(slice1, "H");
  let slice2: &str = &s[3..5];
  assert_eq!(slice2, "lo");
  for (i, c) in s.chars().enumerate() {
      if i == 10 {
          assert_eq!(c, 'd');
      }
  }

  // let utf: &str = "The 🚀 goes to 🌙";
  // let rocket = utf8_slice::slice(utf, 4, 5);
  // will equal "🚀"

  let mut s: String = String::new(); // Vec<u8>
  s.push_str("hello");
  // some bytes, in a vector
  let v: Vec<u8> = vec![104, 101, 108, 108, 111];
  // turns a byte's vector into String
  let s1: String = String::from_utf8(v).unwrap();
  assert_eq!(s, s1);

  let mut s: String = String::with_capacity(25);
  // let mut s: String = String::new();
  println!("{}", s.capacity());
  for _ in 0..2 {
      s.push_str("hello");
      println!("{}", s.capacity()) // 0 8 16
  }

  println!("Success!");
}

fn move_ownership(s: String) {
  println!("Ownership of \"{}\" is move here!", s)
}
