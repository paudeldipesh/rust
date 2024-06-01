#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let x: i32 = 5;
    // Shadowing
    {
        let x = 12;
        assert_eq!(x, 12)
    }
    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);

    let mut y: i32 = 1;
    // Re-binding
    y = 7;
    y += 1;

    let z: i32 = 9;
    let z: &str = "I can be bound to text.";
    print!("Success");
}