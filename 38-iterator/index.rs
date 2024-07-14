use std::collections::HashMap;

pub trait _IteratorTrait {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // Method with default implementations elided
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 0, 1, 1, 2, 3, 5, 8, 13
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let forward = self.curr + self.next;
        self.curr = self.next;
        self.next = forward;
        Some(self.curr)
    }
}

fn main() {
    // iterator
    let v: Vec<i32> = vec![1, 2, 3];
    // v -> v.into_iter() by default
    for x in v.into_iter() {
        println!("{}", x)
    }

    let arr: [i32; 3] = [1; 3]; // [1, 1, 1]
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    let mut i: i32 = 20;
    loop {
        println!("This will print!");
        i += 1;
        if i >= 23 {
            break;
        }
    }

    let mut count: i32 = 1;
    while count <= 5 {
        println!("{}", count);
        count += 1;
    }

    let mut v: Vec<i32> = Vec::new(); // [0, 1, 2, .., 99]
    for n in 0..100 {
        v.push(n)
    }
    assert_eq!(v.len(), 100);

    let mut v1: std::vec::IntoIter<i32> = vec![1, 2].into_iter();
    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);

    let arr: Vec<i32> = vec![7; 10];
    for i in arr.iter() {
        println!("{}", i);
    }
    println!("{:?}", arr);

    let mut names: Vec<&str> = vec!["Bob", "Frank", "Dipesh"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Dipesh" => "There is a rustacean amoung us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();
    if let Some(v) = values_iter.next() {
        *v = 0
    }
    assert_eq!(values, vec![0, 2, 3]);
    for e in values {
        println!("{}", e)
    }

    let mut counter: Counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let mut fib: Fibonacci = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
    assert_eq!(fib.next(), Some(8));

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter: std::slice::Iter<i32> = v1.iter();
    println!("{:?}", v1_iter);
    // The sum method will take the ownership of the iterator and iterates through the
    // items by repeatedly calling next method
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    println!("v1: {:?}", v1);
    println!("total: {}", total);

    let names: [(&str, i32); 2] = [("Dipesh", 23), ("Aman", 25)];
    let folks: HashMap<&str, i32> = names.into_iter().collect();
    println!("{:?}", folks);
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.into_iter().collect();
    assert_eq!(v2, vec![1, 2, 3]);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x: &i32| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    println!("{:?}", v1);
    for e in v2 {
        println!("{}", e)
    }

    println!("Success!")
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}
