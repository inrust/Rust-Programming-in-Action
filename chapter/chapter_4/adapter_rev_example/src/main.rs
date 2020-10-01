fn main() {
    let v = [1, 2, 3];
    let result = v.iter().rev();

    for i in result {
        println!("{}", i);
    }
}
