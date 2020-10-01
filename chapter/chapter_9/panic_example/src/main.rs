use std::panic;

fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    let result = panic::catch_unwind(|| { println!("{}", v[99]) });
    assert!(result.is_err());
    println!("{}", v[1]);
}
