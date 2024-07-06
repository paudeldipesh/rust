use std::{fs, io, num};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseError(e)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    println!("Trying to open file: {}", file_name);
    let contents: String = fs::read_to_string(&file_name)?;
    println!("File contents: {}", contents);
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // Conversion with from/into
    let my_str = "hello";
    let _str1: String = my_str.to_string();
    let _str2: String = my_str.to_string();
    let _str3: String = my_str.into(); // Explicit type annotation is required here

    // impl From<bool> for i32
    let i1: i32 = false.into(); // 0
    let i2 = i32::from(false); // 0
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);
    let _i3: u32 = 'a'.into();
    let _s: String = String::from('a');

    let num1: Number = Number::from(30);
    assert_eq!(num1.value, 30);
    let num2: Number = 30.into();
    assert_eq!(num2.value, 30);

    let file_name: &str = "file_name"; // cargo run file_name (content: number)
    match open_and_parse_file(file_name) {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => match e {
            CliError::IoError(io_error) => println!("IoError: {:?}", io_error.kind()),
            CliError::ParseError(parse_error) => {
                println!("ParseError: {:?}", parse_error.kind())
            }
        },
    }

    let n: i16 = 256;
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("TryFromIntError: {}", e.to_string());
            0
        }
    };
    assert_eq!(n, 0);

    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));
    let result1: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result1, Ok(EvenNum(8)));
    let result2: Result<EvenNum, ()> = 5_i32.try_into();
    assert_eq!(result2, Err(()));

    println!("Success!")
}
