/* Three Rules of Ownership:
   1. Each value in Rust has an owner.
   2. There can only be one owner at a time.
   3. When the owner goes out of scope, the value will be dropped.

   Owner: The owner of a value is the variable or data structure that holds it and is
   responsible for allocating and freeing the memory used to store that data.
*/

fn main() {
  let x: String = String::from("Hello");
  let y: String = x.clone();
  println!("x: {}, y: {}", x, y);

  let s1: String = take_ownership(x.clone());
  println!("{}", s1);

  let get_string: String = give_ownership();
  println!("{}", get_string);

  print_string(x); // x takes the ownership of the value in it.

  let i: (i32, i32, (), &str) = (1, 2, (), "nepal");
  let j: (i32, i32, (), &str) = i;
  println!("{:?}, {:?}", i, j);

  let k: String = String::from("Good, ");
  let mut new_k = k;
  new_k.push_str("Morning!");
  println!("{}", new_k);

  let p: Box<i32> = Box::new(5); // allocate directly on the heap
  let mut q: Box<i32> = Box::new(1);
  *q = 4;
  println!("Memory address of p: {:p}", p);
  assert_eq!(*p, 5);
  assert_eq!(*q, 4);
  println!("Success!");
}

fn take_ownership(str: String) -> String {
  format!("{} Sir!", str)
  // println!("{}", str);
  // str
}

fn give_ownership() -> String {
  let str: String = String::from("How are you?");
  let _str = str.as_bytes();
  str
}

fn print_string(str: String) {
  println!("{}", str)
}