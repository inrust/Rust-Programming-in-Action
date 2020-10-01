use std::rc::Rc;

fn main() {
    let x = Rc::new(5);
    println!("{:p}, count after constructing x: {}", x, Rc::strong_count(&x));
    let y = x.clone();
    println!("{:p}, count after constructing y: {}", y, Rc::strong_count(&x));
    {
        let z = Rc::clone(&x);
        println!("{:p}, count after constructing z: {}", z, Rc::strong_count(&x));
    }

    println!("count after destructing z: {}", Rc::strong_count(&x));
}
