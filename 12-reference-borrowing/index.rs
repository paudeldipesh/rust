#[allow(unused_mut)]
fn main() {
    let x: i32 = 5;
    let p: &i32 = &x;
    println!("The memory address of x is {:p}", p); // One possible output: 0x7ffc74b6dafc
    assert_eq!(5, *p);

    let mut s: String = String::from("Hello World");
    borrow_object(&s);

    let mut greet: String = String::from("Good ");
    push_str(&mut greet);
    println!("{}", greet);

    let mut str: String = String::from("Ram ");
    let str1: &mut String = &mut str;
    str1.push_str("Sita");
    println!("{}", str1); // first
    println!("{}", str); // second

    let c: char = 'd';
    let r1: &char = &c;
    let ref r2 = c;
    println!("{:p}, {:p}", r1, r2); // same address
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    let mut s1: String = String::from("Borrowing Rules");
    let r1 = &s1;
    let r2 = &s1;
    println!("{}, {}", r1, r2);
    println!("s: {}", s1);

    let mut s2: String = String::from("Nepal");
    borrow_object2(&mut s2);

    borrow_object(&s2);
    s2.push_str(" Country!");
    println!("{}", s2);

    let mut s: String = String::from("Hello, ");
    let r1: &mut String = &mut s;
    r1.push_str("World");
    let r2: &mut String = &mut s;
    r2.push_str("!");
    // println!("{}, {}", r1, r2); // cannot borrow `s` as mutable more than once at a time
    println!("{}", r2);

    println!("Success!");
}

#[allow(unused_variables)]
fn borrow_object(s: &String) {}

#[allow(unused_variables)]
fn borrow_object2(s: &mut String) {}

fn push_str(s: &mut String) {
    s.push_str("Afternoon!")
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}