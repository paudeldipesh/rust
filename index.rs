#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

trait IPAddress {
    fn display(&self);
}

struct V4(String);
impl IPAddress for V4 {
    fn display(&self) {
        println!("IPV4: {:?}", self.0);
    }
}
struct V6(String);
impl IPAddress for V6 {
    fn display(&self) {
        println!("IPV6: {:?}", self.0)
    }
}

fn main() {
    // vector
    let arr: [u8; 3] = [1, 2, 3];
    let v: Vec<u8> = Vec::from(arr);
    is_vec(&v);
    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(&v);
    // vec!(..) and vec![..] are the same macros, so
    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(&v);
    let mut v1: Vec<u8> = Vec::new();
    is_vec(&v1);
    for i in &v {
        v1.push(*i);
    }
    assert_eq!(v, v1);

    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    let mut v2: Vec<i32> = Vec::new();
    v2.extend(&v1);
    assert_eq!(v1, v2);

    let arr: [u8; 3] = [1, 2, 3];
    let v1: Vec<u8> = Vec::from(arr);
    let v2: Vec<u8> = arr.into();
    assert_eq!(v1, v2);
    let s: String = "hello".to_string();
    let v1: Vec<u8> = s.into();
    let s: String = "hello".to_string();
    let v2: Vec<u8> = s.into_bytes();
    assert_eq!(v1, v2);
    let s: &str = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]); // [0, 0, 0, .., 0] (10 zeros)

    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i)); // Option<i32>
    }
    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2),
        }
    }
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    let mut v: Vec<i32> = vec![1, 2, 3];
    let slice1: &[i32] = &v[..];
    let slice2: &[i32] = &v[0..v.len()];
    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];
    assert_eq!(slice3, &[1, 2, 3, 4]);

    let mut vec = Vec::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);
    vec.push(10);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    let _v = vec![1, 2, 3];

    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V4("::1".to_string()),
    ];
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V4("::1".to_string()));

    let v: Vec<Box<dyn IPAddress>> = vec![
        Box::new(V4(String::from("127.0.0.1"))),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v {
        ip.display();
    }

    println!("Success!");
}

fn is_vec(_v: &Vec<u8>) {}
