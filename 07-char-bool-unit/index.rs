use std::mem::size_of_val;

fn main() {
    let c1: char = 'w'; // 4 bytes
    assert_eq!(size_of_val(&c1), 4); // size of the memory location
    let c2: char = '中'; // 4 bytes
    assert_eq!(size_of_val(&c2), 4);
    let c3: &str = "Dipesh Paudel"; // 16 bytes
    assert_eq!(size_of_val(&c3), 16);

    let c4: char = 'd';
    print_char(c4);

    let _b1: bool = false;
    let b2: bool = false;
    if !b2 {
        println!("Work!");
    }

    let b3: bool = true;
    let b4: bool = true && true;
    assert_eq!(b3, b4);

    let _v1: () = (); // unit type with empty tuple
    let _v2: (i32, i32) = (1, 2);
    assert_eq!(_v1, implicitly_return_unit_type());

    let unit: () = ();
    assert_eq!(size_of_val(&unit) == 0, true);

    println!("Sucess!");
}

fn print_char(c: char) {
    println!("{}", c);
}

fn implicitly_return_unit_type() {
    println!("This will return unit type");
}

#[allow(dead_code)]
fn explicity_return_unit_type() -> () {
    println!("This will return unit type");
}