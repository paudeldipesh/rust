// Associated types
// &dyn and Box<dyn>
// Static and dynamic dispatch
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying!")
    }
}
impl Bird for Duck {
    fn quack(&self) -> String {
        String::from("duck duck")
    }
}

struct Swan;
#[allow(dead_code)]
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying!")
    }
}
impl Bird for Swan {
    fn quack(&self) -> String {
        String::from("swan swan")
    }
}

trait Draw {
    fn draw(&self) -> String;
}
impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self) // * is not mandatory
    }
}
impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self)
    }
}

trait Foo {
    fn method(&self) -> String;
}
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", self)
    }
}
impl Foo for String {
    fn method(&self) -> String {
        format!("String: {}", self)
    }
}

fn main() {
    let duck: Duck = Duck;
    duck.fly();
    let bird1: Box<dyn Bird> = hatch_a_bird(2);
    assert_eq!(bird1.quack(), "duck duck");
    let bird2: Box<dyn Bird> = hatch_a_bird(1);
    assert_eq!(bird2.quack(), "swan swan");

    let birds: [&dyn Bird; 2] = [&Duck, &Swan]; // usize (size of the pointer)
    for bird in birds {
        println!("{}", bird.quack());
    }

    let x: f64 = 1.1f64;
    let y: u8 = 8_u8;
    draw_with_box(Box::new(x));
    draw_with_ref(&y);

    let x: u8 = 5u8;
    let y: String = String::from("Hello");
    static_dispatch(x);
    dynamic_dispatch(&y); // &String (reference to string)

    println!("Success!")
}

fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

fn static_dispatch<T: Foo>(a: T) {
    a.method();
}

fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}
