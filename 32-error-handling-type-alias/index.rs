use std::{
  fs::File,
  io::{Error, Read},
  num::ParseIntError,
};

type U64 = u64;

fn main() {
  // Result, unwrap, ? operator (almost exactly equivalent to unwrap) and type alias
  let result: Result<f32, &str> = divide(10.0, 2.0);
  match result {
      Ok(value) => println!("Result: {}", value),
      Err(message) => println!("Error: {}", message),
  }

  let _number: U64 = 42;

  let result1: Result<i32, ParseIntError> = multiply("10", "2");
  assert_eq!(result1, Ok(20));
  let result2: Result<i32, ParseIntError> = multiply("4", "2");
  assert_eq!(result2.unwrap(), 8);

  assert_eq!(addition("3", "4").unwrap(), 7);

  // match read_file1() {
  //     Ok(contents) => {
  //         println!("{}", contents);
  //     }
  //     Err(e) => {
  //         println!("{}", e);
  //     }
  // }

  // println!("{}", read_file2().unwrap());

  assert_eq!(
      read_file1().unwrap_err().to_string(),
      read_file2().unwrap_err().to_string()
  );

  assert_eq!(add_two("4").unwrap(), 6);

  let three: Result<i32, ParseIntError> = subtraction_one("5", "2");
  print(three); // OK(3)
  let five: Result<i32, ParseIntError> = subtraction_two("7", "2");
  print(five); // OK(5)

  println!("Success!")
}

fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
  if y == 0.0 {
      return Err("Division by zero");
  }
  Ok(x / y)
}

fn multiply(str1: &str, str2: &str) -> Result<i32, ParseIntError> {
  let n1: Result<i32, ParseIntError> = str1.parse::<i32>();
  let n2: Result<i32, ParseIntError> = str2.parse::<i32>();
  Ok(n1.unwrap() * n2.unwrap()) // Ok(i32) -> i32
}

fn addition(first: &str, second: &str) -> Result<i32, ParseIntError> {
  let first: i32 = first.parse::<i32>()?;
  let second: i32 = second.parse::<i32>()?;
  return Ok(first + second);
}

fn subtraction_one(str1: &str, str2: &str) -> Result<i32, ParseIntError> {
  match str1.parse::<i32>() {
      Ok(first_number) => match str2.parse::<i32>() {
          Ok(second_number) => Ok(first_number - second_number),
          Err(e) => Err(e),
      },
      Err(e) => Err(e),
  }
}

fn subtraction_two(first: &str, second: &str) -> Result<i32, ParseIntError> {
  first
      .parse::<i32>() // OK(7)
      .and_then(|first: i32| second.parse::<i32>().map(|second: i32| first - second))
}

fn read_file1() -> Result<String, Error> {
  let f: Result<File, Error> = File::open("filename.extension");
  let mut f: File = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
  };

  let mut s: String = String::new();
  match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
  }
}

fn read_file2() -> Result<String, Error> {
  let mut s: String = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

// map and and_then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
  // n_str.parse::<i32>().map(|n: i32| n + 2)
  n_str.parse::<i32>().and_then(|n: i32| Ok(n + 2))
}

fn print(result: Result<i32, ParseIntError>) {
  match result {
      Ok(n) => println!("Number: {}", n),
      Err(e) => println!("Error: {}", e),
  }
}
