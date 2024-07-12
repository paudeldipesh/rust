#![allow(dead_code)]
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }

    fn print(&self) {
        println!("`print`: {}", self.0)
    }
}

struct ImportantExcerpt {
    part: &'static str,
}

impl ImportantExcerpt {
    fn level(&self) -> i32 {
        3
    }
}

struct Person {
    age: u8,
    name: &'static str,
}

fn main() {
    // lifetimes
    let i: i32 = 3;
    {
        let borrow1: &i32 = &i; // `borrow1` lifetime starts.
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends.
    {
        let borrow2: &i32 = &i;
        println!("borrow2: {}", borrow2);
    }

    {
        let x: i32 = 5;
        let r: &i32 = &x;
        println!("r: {}", r);
    }

    // {
    //     let r: &i32;
    //     {
    //         let x: i32 = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r)
    // }

    let x: i32 = 7;
    let y: i32 = 9;
    print_one(&x);
    print_multi(&x, &y);
    let z: &i32 = pass_x(&x, &y);
    print_one(z);
    let mut t: i32 = 3;
    add_one(&mut t);
    print_one(&t);

    let x: &str = "long";
    let y: &str = "longer";
    println!("{}", longest(x, y));

    // let x = invalid_output();
    // println!("{}", x);
    let s: String = String::from("foo");
    let x: &str = invalid_output(&s);
    println!("{}", x);

    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    failed_borrow();

    let x: i32 = 18;
    let y: i32 = 15;
    let single: Borrowed = Borrowed(&x);
    let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y };
    let reference: Either = Either::Ref(&x);
    let number: Either = Either::Num(y);
    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("x is *not* borrowed in {:?}", number);

    let var_a: u32 = 35; // 'a
    let example: Example;

    {
        let var_b: NoCopyType = NoCopyType {}; // 'b
        example = Example {
            a: &var_a,
            b: &var_b,
        };
        println!("Example: {:?}", example);
    }
    // var_b goes out of scope here, so 'b' lifetime ends

    let no_copy: NoCopyType = NoCopyType {};
    let example: Example = Example { a: &1, b: &no_copy };
    fix_me(&example);

    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    println!("Success!")
}

// One input reference with lifetime `'a` which must live at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn invalid_output() -> String {
//     String::from("foo")
// }

// fn invalid_output() -> &'static str {
//     "foo"
// }

// &String -> &str
fn invalid_output<'a>(s: &'a str) -> &'a str {
    s
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x: i32 = 12;
    let _y: &i32 = &_x;
}

fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType {
    foo.b
}

fn input(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 {
    x
}

fn longer<'a, 'b>(x: &'a str, _: &'b str) -> &'a str {
    x
}
