fn main() {
    let v = [1, 3, 4, 5];
    let result1 = v.iter().any(|&x| x == 2);
    let result2 = v.iter().any(|x| *x == 2);
    // let result3 = v.iter().any(|x| x == 2);

    println!("{}", result1);
    println!("{}", result2);
}
