#[allow(unused_variables)]
fn main() {
    // String
    let str = String::from("hello world");
    let hello = &str[0..5];
    let world = &str[6..11];
    println!("[{}, {}]", hello, world);

    let s: &str = "dipesh paudel";

    // let str1: Box<str> = "Bye Bye!".into();
    // greetings(&str1);
    let str2: &str = "Bye Bye";
    greetings(str2);

    let mut str3: String = String::from("");
    str3.push_str("Hello");
    str3.push_str(" World");
    assert_eq!(str3, "Hello World");

    let mut str4: String = String::from("I\'");
    str4.push('m');
    str4.push_str(" good");
    str4 += " at Rust.";
    println!("{}", str4);

    let str5: String = String::from("I like dogs");
    let s1 = str5.replace("dogs", "cats"); // replace method
    assert_eq!(s1, "I like cats");

    let s1: String = String::from("Rajesh");
    let s2: String = String::from(" Hamal");
    let s3: String = s1 + s2.as_str(); // String -> &str (&s2)
    assert_eq!(s3, "Rajesh Hamal");
    println!("{}", s3);

    let s4: &str = "Tall Star";
    // greeting(String::from(s4)); // &str -> String
    // greeting(s4.to_string()); // &str -> String
    greeting(s4.to_owned()); // &str -> String

    let s5: String = "Windy Mountain".to_string();
    let s6: &str = &s; // &String -> &str (s.as_str())

    let byte_escape = "I'm writing Ru\x73\x74!"; // bytes by their hexadecimal values
    println!("What are you doing\x3f\n{}", byte_escape);
    let unicode_codepoint = "\u{211D}"; // unicode points
    let character_name = "\"Double-Struck Capital R\"";
    println!(
        "Unicode character {} (U+211D) is {}.",
        unicode_codepoint, character_name
    );
    let long_string = "This is a very long string
that is going \\ to be split
into multiple lines.";
    println!("{}", long_string);

    let raw_str = r"Escape don't work in raw strings: Ru\x73\x74!";
    // assert_eq!(raw_str, "Escape don't work in raw strings: Rust!"); // due to raw strings
    assert_eq!(raw_str, r"Escape don't work in raw strings: Ru\x73\x74!");
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    // String index
    let hi: String = String::from("hi, 中");
    let h1: &str = &hi[0..1];
    assert_eq!(h1, "h");
    let h2: &str = &hi[4..7];
    assert_eq!(h2, "中");

    for c in "你好，世界".chars() {
        println!("{}", c);
    }

    println!("Success!");
}

fn greetings(s: &str) {
    println!("{}", s)
}

fn greeting(s: String) {
    println!("{}", s)
}
