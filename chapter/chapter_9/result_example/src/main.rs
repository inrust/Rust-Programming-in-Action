use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("content: {}", read_from_file_7().unwrap());
}

fn read_from_file_1() {
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open hello.txt: {:?}", error)
        }
    };
}

fn read_from_file_2() {
    let file = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_from_file_3() {
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create hello.txt: {:?}", e),
            },
            other_error => panic!("Failed to open hello.txt: {:?}", other_error),
        },
    };
}

fn read_from_file_4() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed to create hello.txt: {:?}", error);
            })
        } else {
            panic!("Failed to open hello.txt: {:?}", error);
        }
    });
}

fn read_from_file_5() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut file = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_from_file_6() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file_7() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
