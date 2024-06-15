#[allow(unused_variables)]
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);

    let arr1 = [1, 2, 3];
    let arr2: [_; 3] = ["a", "b", "c"];
    assert!(std::mem::size_of_val(&arr1) == 12);
    assert!(std::mem::size_of_val(&arr2) == 48);

    let list: [i32; 100] = [1; 100]; // [1, 1, 1, 1, 1, 1, ..., 1]
    assert!(list[0] == 1);
    assert!(list.len() == 100);

    // all elements in the array must be of the same type ('3' -> char, "3" -> str))
    let _arr3: [i32; 3] = [1, 2, 3];

    let arr4: [char; 3] = ['a', 'b', 'c'];
    let ele = arr4[0];
    assert!(ele == 'a');

    let names: [String; 2] = [String::from("Dipesh"), "Dinesh".to_string()];
    // `get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();
    println!("{}", name0);
    // Indexing is not safe
    // let _name1 = &names[2]; // index out of bounds (Error)

    // Slicing
    let arr5: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr5[0..2];
    let s2: &str = "hello, world";

    let arr6: [char; 3] = ['a', 'b', 'c'];
    let slice: &[char] = &arr6[..2];
    println!("{:?}", slice);
    assert!(std::mem::size_of_val(&slice) == 16);

    let arr7: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr7[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    let str: String = String::from("Hello");
    let slice1: &str = &str[0..2];
    let slice2: &str = &str[..2];
    assert_eq!(slice1, slice2);

    let str1: &str = "你好，世界";
    let slice: &str = &str1[..3];
    assert!(slice == "你");

    let mut str2: String = String::from("Zebra");
    // &str2 is `&String` type, but `first_word` need a `&str` type
    // `&String` is implicitly converted to `&str`
    let word: &str = first_word(&str2); // &String -> &str
    println!("First Word: {}", word); // immutable borrow's output
    str2.clear();

    println!("Success!")
}

fn first_word(s: &str) -> &str {
    &s[..1]
}