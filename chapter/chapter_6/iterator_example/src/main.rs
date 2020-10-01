fn main() {
    let vec = vec!["Java", "Rust", "Python"];
    for str in vec.into_iter() {
        match str {
            "Rust" => println!("Niubility"),
            _ => println!("{}", str),
        }
    }

    let vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter() {
        match str {
            &"Rust" => println!("Niubility"),
            _ => println!("{}", str),
        }
    }

    println!("{:?}", vec);

    let mut vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter_mut() {
        match str {
            &mut "Rust" => {
                *str = "Niubility";
                println!("{}", str);
            },
            _ => println!("{}", str),
        }
    }

    println!("{:?}", vec);
}
