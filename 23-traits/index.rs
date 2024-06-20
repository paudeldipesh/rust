use std::ops;
trait Hello {
    fn say_hi(&self) -> String {
        String::from("Hi")
    }

    fn say_something(&self) -> String;
}

struct Student;
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student.")
    }
}
struct Teacher;
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher.")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher.")
    }
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self; // Inches(12)
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

fn multiply<T: ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b // a.mul(b)
}

struct Foo;
struct Bar;

#[derive(Debug, PartialEq)]
struct FooBar;
#[derive(Debug, PartialEq)]
struct BarFoo;

// Foo + Bar => Foo.add(Bar)
// Foo + Foo => Foo.add(Foo) (Self is used in the place of Bar)
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

// Foo - Bar => Foo.sub(Bar)
impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;
    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of the post {} is {}.", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}.", self.username, self.content)
    }
}

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "Baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "Moooooh!".to_string()
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

fn main() {
    // traits
    let s: Student = Student {};
    assert_eq!(s.say_hi(), "Hi");
    assert_eq!(s.say_something(), "I'm a good student.");

    let t: Teacher = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher.");
    assert_eq!(t.say_something(), "I'm not a bad teacher.");

    let one_seconds: Seconds = Seconds(1);
    println!("One seconds looks like {:?}", one_seconds);
    let _this_is_true: bool = one_seconds == one_seconds;
    let _this_is_false: bool = one_seconds > one_seconds;
    let foot: Inches = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter: Centimeters = Centimeters(100.0);
    // type Centimeters
    let cmp: &str = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter.", cmp);

    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    let post: Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo: Weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };
    summary(&post);
    summary(&weibo);
    println!("{:?}", post);
    println!("{:?}", weibo);

    let random_number: f64 = 0.534;
    let animal: Box<dyn Animal> = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    assert_eq!(sum(1, 2), 3);

    let pair: Pair<Unit> = Pair::new(Unit(1), Unit(3));
    pair.cmp_display();

    println!("Success!");
}

// fn summary(a: &impl Summary) (same output)
fn summary<T: Summary>(a: &T) {
    let output: String = a.summarize();
    println!("{}", output)
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn sum<T: ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y // x.add(y)
}
