fn main() {
  #[derive(Debug)]
  struct Person {
      name: String,
      age: Box<u8>,
  }

  let person: Person = Person {
      name: String::from("John"),
      age: Box::new(30),
  };

  // 'name' is moved out of person, but 'age' is referenced
  let Person { name, ref age } = person;
  println!("{} is {} years old.", name, age);
  // Error: borrow of partiallly moved value: `person`
  // println!("The person struct is {:?}", person);
  println!("The age is {}.", person.age);

  let t1: (String, String) = (String::from("hello"), String::from("world"));
  let _ta: String = t1.0;
  println!("{:?}", t1.1);

  let t2: (String, String) = (String::from("good"), String::from("night"));
  let (a, b) = t2.clone();
  println!("[{:?}, {:?}, {:?}]", a, b, t2);
}