use std::fmt::Display;

struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

impl<'a, 'b> Foo<'a, 'b> {
    fn x(&self) -> &i32 { self.x }

    fn y(&self) -> &i32 { self.y }

    fn max_x(&'a self, f: &'a Foo) -> &'a i32 {
        if self.x > f.x {
            self.x
        } else {
            f.x
        }
    }
}

fn main() {
    let i = 7;
    let r = &i;
    println!("r: {}", r);

    let f1 = Foo { x: &3, y: &5 };
    let f2 = Foo { x: &7, y: &9 };
    println!("x: {}, y: {}", f1.x(), f1.y());
    println!("max_x: {}", f1.max_x(&f2));

    let str1 = String::from("abcd");
    let result;
    {
        let str2 = "xyz";
        result = long_str(str1.as_str(), str2);
    }
    println!("longer string: {}", result);

    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = long_str_with_tip(str1.as_str(), str2, 777);
    println!("longer string: {}", result);
}

fn long_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn long_str_with_tip<'a, T>(x: &'a str, y: &'a str, tip: T) -> &'a str
    where T: Display
{
    println!("Tip: {}", tip);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
