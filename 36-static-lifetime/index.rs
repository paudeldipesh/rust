use std::fmt::{Debug, Display};
use std::sync::{Mutex, MutexGuard, Once};

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<Mutex<Config>> = None;
static INIT: Once = Once::new();
// NUM is stored in a fixed memory location for the duration of the program
static NUM: i32 = 18; // static and const are written in the uppercase

fn main() {
    // Static lifetime
    let _s: &'static str = "hello world";
    #[allow(dead_code)]
    fn generic<T>(_x: T)
    where
        T: 'static,
    {
    }

    let v: &str = "hello";
    need_static(v);

    {
        let config: MutexGuard<Config> = get_config().lock().unwrap();
        println!("{:?}", *config);
    }

    let static_string: &'static str = "I'm in read-only memory";
    {
        println!("`static_string`: {}", static_string);
        // When `static_string` goes out of scope, the reference can no longer be used, but
        // the data remains in the binary.
    }
    println!("`static_string` reference remains alive: {}", static_string);

    {
        let lifetime_num: i32 = 9;
        let coerced_static: &i32 = coerce_static(&lifetime_num);
        println!("`coerced_static`: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);

    // const I: i32 = 5;
    static I: i32 = 5;
    print_it(I);
    // oops, &I only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&I);
    print_it1(&I);
    print_it2(&I);

    let mut string: String = "first".to_owned();
    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(&string);
    print_d(&string);
    print_e(&string);
    print_f(&string);
    print_g(&string);

    println!("Success!");
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}

fn get_config() -> &'static Mutex<Config> {
    unsafe {
        INIT.call_once(|| {
            let config: Config = Config {
                a: "A".to_string(),
                b: "B".to_string(),
            };
            CONFIG = Some(Mutex::new(config));
        });
        CONFIG.as_ref().unwrap()
    }
}

// 'static > 'a
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input)
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input)
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input)
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t)
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t)
}

fn print_c(t: &dyn Display) {
    println!("{}", t)
}

fn print_d(t: &impl Display) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &String) {
    println!("{}", t)
}
