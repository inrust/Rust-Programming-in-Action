#[derive(Debug)]
enum ColorNoParam {
    Red,
    Yellow,
    Blue,
}

#[derive(Debug)]
enum ColorParam {
    Red(String),
    Yellow(String),
    Blue(String),
}

fn main() {
    let color_no_param = ColorNoParam::Red;
    match color_no_param {
        ColorNoParam::Red => println!("{:?}", ColorNoParam::Red),
        ColorNoParam::Yellow => println!("{:?}", ColorNoParam::Yellow),
        ColorNoParam::Blue => println!("{:?}", ColorNoParam::Blue),
    }

    println!("{:?}", ColorParam::Blue(String::from("blue")));
}
