/*
  Borrowing:
  * Way of temporarily access data without taking ownership of it.
  * When borrowing, you're taking a reference (pointer) to the data, not the data itself.
  * Prevention of dangling pointers and data races.
  * Data can be borrowed immutabily and mutably.
  * There are certain rules when borrowing which we have to comply with, otherwise the program won't compile.

  Rules of references:
  1. At any given time, you can have either one mutable reference or any number of immutable references.
  2. References must always be valid.

  Borrower: The entity (variable or expression) that holds a reference to the borrowed data.
*/

fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);

  let mut s2 = String::from("Rajesh");
  change(&mut s2);

  let mut s3 = String::from("Ram");
  // one mutable reference to the same data at a time!
  let r1 = &mut s3;
  println!("{}", r1);
  // let r2 = &mut s3;
  // println!("{}", r2);

  // Mutable borrowing
  let mut str = String::from("Krishna");
  {
      let str1 = &mut str;
      println!("First one: {}", str1);
  } // str1 goes out of scope here and is dropped.
  let str2 = &str;
  println!("Second one: {}", str2);

  // Immutable borrowing
  let mut str_ref = String::from("Namaskar");
  let str_ref1 = &str_ref;
  let str_ref2 = &str_ref;
  println!("[{}, {}]", str_ref1, str_ref2);
  // let str_mut = &mut str_ref;
  // println!("[{}, {}, {}]", str_ref1, str_ref2, str_mut); // cannot borrow as mutable
  let str_mut = &mut str_ref;
  println!("{}", str_mut);

  let reference_to_nothing = no_dangle();
  println!("{}", reference_to_nothing);
}

// Immutability:
fn calculate_length(s: &String) -> usize {
  s.len()
}

// Mutability:
fn change(some_string: &mut String) {
  some_string.push_str(" Hamal");
  println!("{}", some_string);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
  let s = String::from("No dangling, thank you!");
  s
}