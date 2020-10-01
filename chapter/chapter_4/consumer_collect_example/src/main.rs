fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v2);
}
