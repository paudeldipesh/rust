use std::mem;

fn main() {
    // Closures: one inline function, takes ownership of a value by using `move` keyword.
    // Function: FnOnce, FnMut and Fn.
    let x: i32 = 1;
    // Fn
    let closure = |value: i32| value + x; // one liner anonymous function (closure)
    assert_eq!(closure(2), 3);

    fn function(value: i32) -> i32 {
        value + 1
    }
    let closure_annoted = |num: i32| -> i32 { num + 1 };
    let closure_inferred = |num: i32| num + 1;
    let i: i32 = 1;
    println!("`function`: {}", function(i));
    println!("`closure_annoted`: {}", closure_annoted(i));
    println!("`closure_inferred`: {}", closure_inferred(i));
    let one = || 1;
    println!("`one`: {}", one());

    // Capturing: value, mutable and immutable references
    let color: String = String::from("green");
    let print = || println!("`color`: {}", color);
    print();
    print();
    let _reborrow: &String = &color;
    println!("{}", color);

    // FnMut
    let mut count: i32 = 0;
    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    let _reborrow: &i32 = &count;
    println!("{}", _reborrow);
    inc();
    let _count_reborrowed: &mut i32 = &mut count;
    println!("{}", _count_reborrowed);
    assert_eq!(count, 0);

    // FnOnce
    let movable: Box<i32> = Box::new(3);
    // Clone the movable value inside the closure
    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable); // take reference to argument (movable) to use consume again
    };
    consume();
    // consume(); // run after taking the reference

    // For comparison
    let movable: Box<i32> = Box::new(4);
    let consume = move || {
        println!("`movable`: {:?}", movable);
    };
    consume();
    consume();

    let example_closure = |x: String| -> String { x };
    let s: String = example_closure(String::from("Dipesh Sir"));
    println!("{}", s);
    let i: String = example_closure(5.to_string());
    println!("{}", i);

    let x: Vec<i32> = vec![1, 2, 3];
    fn_once(|z: usize| z == x.len());

    let mut s: String = String::new();
    let update_string = |str| s.push_str(str);
    execute(update_string);

    println!("{:?}", apply(print_dipesh));
    let greeting: &str = "hello";
    let mut farewell: String = "goodbye".to_owned();
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        mem::drop(farewell);
    };
    apply(diary);
    let double = |x: i32| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

    // Move closures may still implement Fn or FnMut, even though they capture variables by move.
    let s: String = String::new();
    let update_string = move || println!("{}", s);
    exec1(update_string);

    let mut s: String = String::new();
    let update_string = |str: &str| -> String {
        s.push_str(&str);
        s
    };
    exec2(update_string);

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(call_fn);

    let plain_fn = create_fn();
    let result = plain_fn(1); // 6
    println!("{}", result);

    println!("Success!")
}

fn take<T>(_v: T) {} // take reference to parameter (T) to use consume again

fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    // The value here should be true if the length matches and false otherwise
    println!("{}", func(3)); // This will print true because x.len() is 3
    println!("{}", func(4)); // This will print false because x.len() is not 4
}

// fn execute<'a, F: FnMut(&'a str)>(mut f: F) {
//     f("Dipesh Paudel is executed.")
// }

fn execute<'a, F: FnOnce(&'a str)>(f: F) {
    f("Dipesh Paudel is executed.")
}

fn apply<F>(f: F)
where
    // The closure takes no input and returns nothing.
    F: FnOnce(),
{
    f();
}

fn print_dipesh() {}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn exec1<F: FnOnce()>(f: F) {
    f()
}

fn exec2<F: FnOnce(&str) -> String>(f: F) {
    f("rust programming");
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn call_fn() {
    println!("I'm a function!");
}

fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num: i32 = 5;
    Box::new(move |x: i32| x + num)
}

// Another approach
// fn create_fn() -> impl Fn(i32) -> i32 {
//     let num: i32 = 5;
//     move |x: i32| x + num
// }

fn _factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num: i32 = 5;
    if x > 1 {
        Box::new(move |x| x + num) // different memory location
    } else {
        Box::new(move |x| x + num) // different memory location
    }
}
