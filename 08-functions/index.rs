#[allow(unreachable_code)]
fn main() {
    let x: u32 = 5u32;
    let y: u32 = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;
        // This expression will be assigned to 'y'
        x_cubed + x_squared + x
    };
    let z: u32 = {
        // The semicolon suppresses this expression and `()` is assigned to 'z'
        2 * x
    };
    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);

    let v1: i32 = {
        let mut x: i32 = 1;
        x += 2; // 3
        x
    };
    assert_eq!(v1, 3);

    let v1: () = {
        let mut _x: i32 = 1; // Declare x within the block
        _x += 2; // Modify x here
    };
    assert_eq!(v1, ());

    let g: i32 = {
        let x: i32 = 3;
        x
    };
    assert!(g == 3);

    let s: i32 = sum(1, 2);
    assert_eq!(s, 3);

    let (i, j) = (3, 4);
    let s: i32 = sum(i, j);
    assert_eq!(s, 7);
    print();

    let b = 3 == 2 + 1;
    let _data = match b {
        true => 1,
        // Diverging functions can also be used in match expression
        false => {
            println!("Success!");
            panic!("We have no value for `false`, but we can panic")
        }
    };
    println!("Failed to print the line");

    never_return_fn();
    println!("From never return fn!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// implicitly returns ()
fn print() {
    println!("Hello World");
}

#[allow(dead_code)]
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // Todo
        }
        _ => {
            // Todo
        }
    };
    never_return_fn();
}

// Three ways to implement a function
fn never_return_fn() -> ! {
    // todo!();
    // unimplemented!();
    panic!();
}