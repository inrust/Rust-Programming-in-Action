fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    v[1] = 5;
    println!("v: {:?}", v);

    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    v.push(2);
    v.push(3);

    println!("e: {:?}", v.pop());
    println!("v: {:?}", v);

    let mut v = vec![1, 2, 3];

    println!("e: {}", v.remove(1));
    println!("v: {:?}", v);

    let v = vec![1, 2, 3];
    println!("e: {}", v[2]);
    // println!("e: {}", v[10]);

    println!("e: {:?}", v.get(2));
    println!("e: {:?}", v.get(10));

    let v = vec![10, 20, 30];
    for i in v {
        print!("{} ", i);
    }

    let mut v = vec![10, 20, 30];
    for i in &mut v {
        *i += 50;
        print!("{} ", i);
    }
}
