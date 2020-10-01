use std::cell::RefCell;

fn main() {
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", v.borrow());

    v.borrow_mut().push(5);
    println!("{:?}", v.borrow());
}
