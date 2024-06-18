#![allow(dead_code)]

struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

struct PointOne<T> {
    x: T,
    y: T,
}

impl PointOne<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwo<T1, T2> {
    x: T1,
    y: T2,
}

struct Value<T> {
    value: T,
}

impl<T> Value<T> {
    fn value(&self) -> &T {
        &self.value
    }
}

impl<T1, T2> PointTwo<T1, T2> {
    fn mixup<T3, T4>(self, other: PointTwo<T3, T4>) -> PointTwo<T1, T4> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

struct Array<T, const N: usize> {
    data: [T; N],
}

#[allow(unused_variables)]
fn main() {
    // using non-generic functions
    reg_fn(S(A)); // concrete type
    gen_spec_t(SGen(A)); // implicitly specified type parameter `A`
    gen_spec_i32(SGen(42)); // implicitly specified type parameter `i32`

    // explicitly specified type parameter `char` to `generic`
    generic::<char>(SGen('a'));
    // implicitly specified type parameter `f64` to `generic`
    generic(SGen(10.0));
    // implicitly specified type parameter `&str` to `generic`
    generic(SGen("Dipesh Paudel"));

    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30)); // i32
    assert_eq!(2.46, sum(1.23, 1.23)); // f64

    let integer: PointOne<i32> = PointOne { x: 5, y: 10 };
    let float: PointOne<f64> = PointOne { x: 1.0, y: 2.0 };

    let ps: PointTwo<i32, String> = PointTwo {
        x: 5,
        y: String::from("hi"),
    };

    let x: Value<f64> = Value { value: 3.0 };
    let y: Value<String> = Value {
        value: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());

    let p1: PointTwo<i32, i32> = PointTwo { x: 5, y: 10 };
    let p2: PointTwo<&str, char> = PointTwo { x: "Hello", y: 'd' };
    let p3: PointTwo<i32, char> = p1.mixup(p2);
    // PointTwo { x: 5, y: 'd' }
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 'd');

    let p: PointOne<f64> = PointOne { x: 5.0, y: 10.0 };
    println!("{}", p.distance_from_origin());

    let integer_arrays: [Array<i32, 3>; 3] = [
        Array { data: [1, 2, 3] },
        Array { data: [4, 5, 6] },
        Array { data: [7, 8, 9] },
    ];
    let float_arrays: [Array<f64, 2>; 3] = [
        Array { data: [1.0, 2.0] },
        Array { data: [3.0, 4.0] },
        Array { data: [5.0, 6.0] },
    ];

    println!("Success");
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
