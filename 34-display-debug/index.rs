use std::fmt;

// This struct does not implement Debug or Display, so it is not printable by default.
struct _UnPrintable(i32);
#[derive(Debug)]
// This struct derives the Debug trait, making it printable using the {:?} formatter.
struct _DebugPrintable(i32);

struct Structure<T>(T);
struct Deep<T>(Structure<T>);
impl<T: fmt::Debug> fmt::Debug for Deep<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0 .0)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Debug: Complex {{ real: {} + imag: {} }}",
            self.x, self.y
        )
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec: &Vec<i32> = &self.0;
        write!(f, "[")?;
        for (index, value) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", index, value)?;
        }
        write!(f, "]")
    }
}

fn main() {
    // Debug and Display
    let s1: &str = "hello";
    let s: String = format!("{}, world!", s1);
    println!("{}", s);

    print!("hello world, ");
    println!("I am"); // \n
    println!("Dipesh Paudel!");

    println!("{} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3).0);
    println!("Now {:?} will print!", Structure(7.0).0);

    let person: Person = Person {
        name: "Dipesh Paudel".to_string(),
        age: 23,
    };
    println!("{:#?}", person);

    println!("i32 is {:?}", Deep(Structure(69)));
    println!("f64 is {:?}", Deep(Structure(69.69)));

    let point: Point2D = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(
        format!("{:?}", point),
        "Debug: Complex { real: 3.3 + imag: 7.2 }"
    );

    let v: List = List(vec![1, 2, 3]);
    assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");

    println!("Success!")
}
