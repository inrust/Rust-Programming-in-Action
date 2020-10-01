fn main() {
    let x: u16 = 7;
    let y = x as u32;
    println!("u16: {}, u32: {}", x, y);

    let x = std::u32::MAX;
    let y = x as u16;
    println!("u32: {}, u16: {}", x, y);

    let x = 65u8;
    let y = x as char;
    println!("u8: {}, char: {}", x, y);

    let x = 'A';
    let y = x as u8;
    println!("char: {}, u8: {}", x, y);

    let x = 7;
    let y = x as f64;
    println!("i32: {}, f64: {}", x, y);

    let x = 7.7;
    let y = x as i32;
    println!("f64: {}, i32: {}", x, y);

    let x = 7;
    let y = x.to_string();
    println!("i32: {}, String: {}", x, y);

    let x = 7.7;
    let y = x.to_string();
    println!("f64: {}, String: {}", x, y);

    let x = String::from("7");
    let y = x.parse::<i32>().unwrap();
    println!("String: {}, i32: {}", x, y);

    let x = String::from("7.7");
    let y = x.parse::<f64>().unwrap();
    println!("String: {}, f64: {}", x, y);

    let x = String::from("hello");
    let y = x.as_str();
    println!("String: {}, &str: {}", x, y);

    let x = "hello";
    let y = x.to_string();
    println!("&str: {}, String: {}", x, y);
}
