#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

struct Unit;
#[allow(dead_code)]
trait SomeTrait {
    // ...Some behaviors defined here
}
impl SomeTrait for Unit {}

struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let age: u8 = 30;
    let _p: Person = Person {
        name: String::from("Dipesh"),
        age,
        hobby: String::from("Coding"),
    };

    let u = Unit;
    do_something_with_unit(u);

    let v: Color = Color(0, 127, 255);
    check_color(v);

    let age: u8 = 19;
    let mut e: Employee = Employee {
        name: String::from("Sunface"),
        age,
    };

    e.age = 20;
    e.name = String::from("Dipesh");
    print_employee(&e);

    let user1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    let user2: User = set_email(user1);
    println!("{:?}", user2);

    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        // print debug info to Standard Error and assign the value of `30 * scale` to `width`
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1); // print debug info to Standard Error
    println!("{:?}", rect1); // print debug info to Standard Output

    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Dipesh Paudel".to_string(),
    };
    let _name: String = f.name; // use clone method to copy the string
    println!("FileInfo: [{}, {}]", _name, f.data);

    println!("Success!")
}

fn do_something_with_unit(_u: Unit) {}

fn check_color(p: Color) {
    let Color(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

fn print_employee(e: &Employee) {
    println!("Employee: {:?}", e);
}

#[allow(dead_code)]
fn build_empolyee(name: String, age: u8) -> Employee {
    Employee { name, age }
}

fn set_email(user: User) -> User {
    User {
        email: String::from("anotherone@example.com"),
        ..user
    }
}