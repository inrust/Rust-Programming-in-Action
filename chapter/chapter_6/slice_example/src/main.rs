fn main() {
    let s = String::from("Hello, Rust!");
    println!("{}", &s[0..5]);
    println!("{}", &s[..5]);
    println!("{}", &s[7..s.len()]);
    println!("{}", &s[7..]);
    println!("{}", &s[0..s.len()]);
    println!("{}", &s[..]);

    let vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", &vec[0..2]);
    println!("{:?}", &vec[..2]);
    println!("{:?}", &vec[2..vec.len()]);
    println!("{:?}", &vec[2..]);
    println!("{:?}", &vec[0..vec.len()]);
    println!("{:?}", &vec[..]);

    let str = "Hello";
    print_str(&s[0..5]);
    print_str(&str);
    print_vec(&vec[2..]);

    let mut vec = vec![1, 2, 3, 4, 5];
    let vec_slice = &mut vec[3..];
    vec_slice[0] = 7;
    println!("{:?}", vec);
}

fn print_str(s: &str) {
    println!("slice: {:?}, length: {}", s, s.len());
}

fn print_vec(vec: &[i32]) {
    println!("slice: {:?}, length: {}", vec, vec.len());
}
