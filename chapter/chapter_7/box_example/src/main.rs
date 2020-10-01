fn main() {
    let x:Box<i32> = Box::new(5);
    let y:Box<i32> = x;

    println!("x: {}", x);
    println!("y: {}", y);
}
