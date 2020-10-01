fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 5;
    let y = {
        let x = 2;
        x + 1
    };

    let sum = add(x, y);
    println!("{} + {} = {}", x, y, sum);
}
