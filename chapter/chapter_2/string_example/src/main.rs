fn main() {
    let mut s = String::from("Hello, ");
    s.push('R');
    s.push_str("ust!");
    println!("{}", s);

    let mut s = String::from("Hello World!");
    s.insert(5, ',');
    s.insert_str(7, "Rust ");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from(", ");
    let s3 = String::from("Rust ");
    let s4 = "World";
    let mut s = s1 + &s2 + s3.as_str() + s4;
    s += "!";
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = "World";
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    let s = String::from("aaabbbbccaadd");
    let s1 = s.replace("aa", "77");
    let s2 = s.replacen("aa", "77", 1);

    println!("{}", s1);
    println!("{}", s2);

    let mut s = String::from("Löwe 老虎 Léopard");

    println!("{:?}", s.pop());
    println!("{}", s);

    println!("{:?}", s.remove(9));
    println!("{}", s);

    s.truncate(9);
    println!("{}", s);

    s.clear();
    println!("{}", s);

    let s = String::from("Löwe 老虎");
    println!("Löwe 老虎: {}", s.len());

    let s = String::from("L");
    println!("L: {}", s.len());

    let s = String::from("ö");
    println!("ö: {}", s.len());

    let s = String::from("老");
    println!("老: {}", s.len());

    let s = String::from("Löwe 老虎");
    let bytes = s.bytes();
    for b in bytes {
        print!("{} | ", b);
    }

    println!();

    let chars = s.chars();
    for c in chars {
        print!("{} | ", c);
    }
}
