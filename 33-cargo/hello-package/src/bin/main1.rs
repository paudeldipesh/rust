use hello_package::{
    eat_at_restaurant,
    front_of_house::{hosting::seat_at_table, serving::take_order},
};

fn main() {
    assert_eq!(eat_at_restaurant(), "Yummy Yummy!");
    assert_eq!(take_order(), "Taking order");
    println!("{:?}", seat_at_table());
    println!("Success!")
}
