use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Foo {
    x: i32,
    y: bool,
}

fn main() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let foo = Foo { x: 8, y: true };
    let other = foo;
    println!("foo: {:?}, other: {:?}", foo, other);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let key = String::from("Favorite color");
    let value = String::from("Red");

    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{}", map[&key]);

    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    make_copy(x);

    let s1 = give_ownership();
    let s2 = take_and_give_back(s1);
    println!("{}", s2);
}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn make_copy(int: i32) {
    println!("{}", int);
}

fn give_ownership() -> String {
    let str = String::from("ownership");
    str
}

fn take_and_give_back(name: String) -> String {
    let hello = String::from("hello");
    hello + " " + &name
}
