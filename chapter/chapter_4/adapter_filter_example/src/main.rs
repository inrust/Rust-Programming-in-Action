fn main() {
    let v = [1, 2, 3];

    let result: Vec<i32> = v.iter()
        .map(|x| x + 3)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("{:?}", result);
}
