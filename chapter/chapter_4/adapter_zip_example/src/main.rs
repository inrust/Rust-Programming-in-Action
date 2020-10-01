fn main() {
    let v1 = [1, 2, 3];
    let v2 = [2, 3, 6];

    let result: Vec<i32> = v1.iter().zip(v2.iter())
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("{:?}", result);
}
