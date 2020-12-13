#[derive(Default, Debug)]
struct MyStruct {
    foo: i32,
    bar: f32,
}

fn main() {
    let options1: MyStruct = Default::default();
    let options2 = MyStruct { foo: 7, ..Default::default() };

    println!("options1: {:?}", options1);
    println!("options2: {:?}", options2);
}