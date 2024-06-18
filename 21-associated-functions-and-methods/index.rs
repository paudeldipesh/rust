#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct RectangleOne {
    p1: Point,
    p2: Point,
}

impl RectangleOne {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second)
    }
}

struct RectangleTwo {
    width: u32,
    height: u32,
}

impl RectangleTwo {
    fn area(&self) -> u32 {
      self.width * self.height
    }
}

impl RectangleTwo {
    fn can_hold(&self, other: &RectangleTwo) -> bool{
      self.width > other.width && self.height > other.height
    }
}

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(self: &mut Self) {
        println!("The current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = String::from("green")
    }

    pub fn new() -> Self {
        Self {
            color: String::from("red"),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

enum TrafficLightColor {
  Red, Yellow, Green
}

impl TrafficLightColor {
  fn color(&self) -> String {
    match self {
      TrafficLightColor::Red => String::from("red"),
      TrafficLightColor::Yellow => String::from("yellow"),
      Self::Green => String::from("green")
    }
  }
}

fn main() {
    // Associated Functions & Methods
    let rectangle: RectangleOne = RectangleOne {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square: RectangleOne = RectangleOne {
        p1: Point::origin(),
        p2: Point::new(3.0, 3.0),
    };

    square.translate(1.0, 1.0);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    let rect2: RectangleTwo = RectangleTwo {
        width: 30,
        height: 50,
    };
    assert_eq!(rect2.area(), 1500);

    let light: TrafficLight = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    let c: TrafficLightColor = TrafficLightColor::Red;
    println!("Color: {:?}", c.color());
    assert_eq!(c.color(), String::from("red"));

    println!("Success!");
}