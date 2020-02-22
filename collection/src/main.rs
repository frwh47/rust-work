use std::collections::HashMap;

fn main() {
    test_vec();
    test_string();
    test_hashmap();
}

fn test_vec() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    println!("{}, {:}", v[0], &v[1]);
    println!("{:?}\n", v);

    let v = vec![1, 2, 3, 4, 5, 6, 7];
    for i in &v {
        print!("{:?}, ", i);
    }
    println!("\n{:?}", v);

    let mut v = v;
    for i in &mut v {
        *i *= 10;
    }
    println!("{:?}", v);
}

fn test_string() {
    let hello = "你好!";
    println!("{}", hello);
    let hello = String::from("你好!");
    println!("{}", hello);

    let mut s = String::from("hello");
    s.push_str(" world ");
    println!("{}", s);
    s.push('!');
    println!("{}", s);

    let s1 = "hello";
    let s2 = "world";
    let s3 = "!";
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s1 = String::from(s1);
    let s = s1 + &s2;
    println!("{}", s);
}

fn test_hashmap() {
    let key = "rust";
    let value = 80;
    let mut map = HashMap::new();
    map.insert(key, value);
    println!("{:?}", map);
    println!("{} = {}", key, value);

    let k2 = String::from("Java");
    let v2 = 95;
    let mut map = HashMap::new();
    map.insert(k2, v2);
    println!("{:?}", map);
    println!("{}", v2);
}
