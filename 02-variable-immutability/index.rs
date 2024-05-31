fn main() {
  let x: i32 = 5;
  let _y: i32;

  assert_eq!(x, 5);
  println!("Success");
  println!("{}", x);

  let mut x: i32 = 9;
  x += 2;

  assert_eq!(x, 11);
  println!("Success");
  println!("{}", x);

  let z: i32 = 7;

  {
      let z: i32 = 10;
      println!("x is {} and z is {}", x, z);
  }

  println!("x is {} and z is {}", x, z);
  define_name();
}

fn define_name() {
  let x: &str = "Dipesh";
  print!("{} Paudel", x);
}