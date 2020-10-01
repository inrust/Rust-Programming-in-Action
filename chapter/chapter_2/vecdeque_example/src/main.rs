use std::collections::VecDeque;

fn main() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(1);
    v.push_front(2);

    v[1] = 5;
    println!("v: {:?}", v);

    println!("e: {:?}", v.pop_back());
    println!("e: {:?}", v.pop_front());
    println!("v: {:?}", v);

    let mut v: VecDeque<u32> = VecDeque::with_capacity(10);
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);

    println!("e: {:?}", v.remove(1));
    println!("e: {:?}", v.remove(5));
    println!("v: {:?}", v);

    println!("e: {}", v[0]);
    // println!("e: {}", v[10]);

    println!("e: {:?}", v.get(0));
    println!("e: {:?}", v.get(10));
}
