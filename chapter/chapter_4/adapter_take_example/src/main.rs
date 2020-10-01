fn main() {
    let v = [1, 2, 3, 4, 5];
    let result = v.iter().take(3);

    for i in result {
        println!("{}", i);
    }
}
