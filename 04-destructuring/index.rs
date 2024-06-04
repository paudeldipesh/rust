fn main() {
  let (mut x, y) = (1, 2);
  x += 2;
  assert_eq!(x, 3);
  assert_eq!(y, 2);
  println!("Success");

  let (a, c);
  (a, ..) = (7, 8, 9);
  [.., c] = [1, 2, 3];
  assert_eq!([a, c], [7, 3]);
  println!("Success");
}