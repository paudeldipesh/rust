fn main() {
  // if else condition
  let num: i32 = 5;
  if num < 0 {
      println!("{} is negative", num);
  } else if num > 0 {
      println!("{} is positive", num);
  } else {
      println!("{} is zero", num)
  }

  let new: i32 = -11;
  let big_n: i32 = if new < 10 && new > -10 {
      println!("Small number, increase ten-fold");
      10 * new
  } else {
      println!("Big number, halve the number");
      new / 2.0 as i32
  };
  println!("{} -> {}", new, big_n);

  // for loop
  for n in 1..100 {
      if n == 100 {
          panic!("Never let this run")
      }
  }

  let names: [String; 2] = [String::from("James"), "Bond".to_string()];
  for name in names.clone() {
      println!("{}", name)
  }
  println!("{:?}", names);

  let numbers: [i32; 3] = [1, 2, 3];
  for number in numbers {
      println!("{}", number);
  }
  println!("{:?}", numbers);

  let a: [i32; 4] = [4, 3, 2, 1];
  for (index, value) in a.iter().enumerate() {
      println!("The {}th element is {}", index + 1, value);
  }

  // while loop
  let mut number: i32 = 1;
  while number < 16 {
      if number % 15 == 0 {
          println!("fizzbuzz");
      } else if number % 3 == 0 {
          println!("fizz");
      } else if number % 5 == 0 {
          println!("buzz");
      } else {
          println!("{}", number);
      }
      number += 1;
  }

  // continue and break
  let mut n: i32 = 0;
  for _i in 0..=100 {
      if n == 10 {
          break;
      }
      n += 1;
  }
  assert_eq!(n, 10);

  let mut n: i32 = 10;
  for _i in 0..=10 {
      if n != 12 {
          n += 1;
          println!("{}", n);
          continue;
      }
      break;
  }
  assert_eq!(n, 12);

  // loop (infinity loop)
  let mut count: u32 = 0_u32;
  println!("Let's count to infinity");
  loop {
      count += 1;

      if count == 3 {
          println!("Three");
          continue;
      }

      if count == 5 {
          println!("Five");
          break;
      }

      println!("{}", count);
  }
  assert_eq!(count, 5);

  let mut counter: i32 = 0;
  let result: i32 = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };
  assert_eq!(result, 20);

  // loop with labels
  let mut count: i32 = 0;
  'outer: loop {
      'inner1: loop {
          if count >= 20 {
              break 'inner1;
          }
          count += 2;
      }

      count += 5;

      #[allow(unused_labels)]
      'inner2: loop {
          if count >= 30 {
              break 'outer;
          }
          continue 'outer;
      }
  }
  assert!(count == 30);
  
  println!("Success!")
}