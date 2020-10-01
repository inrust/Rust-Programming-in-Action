fn main() {
    let integer1: u32 = 17;
    let integer2 = 17u32;
    let integer3 = 17;
    let integer4: u32 = 0b10001;
    let integer5: u32 = 0o21;
    let integer6: u32 = 0x11;
    let integer7 = 50_000;

    println!("{}", integer1);
    println!("{}", integer2);
    println!("{}", integer3);
    println!("{}", integer4);
    println!("{}", integer5);
    println!("{}", integer6);
    println!("{}", integer7);

    let float1: f32 = 1.1;
    let float2 = 2.2f32;
    let float3 = 3.3;
    let float4 = 11_000.555_001;

    println!("{}", float1);
    println!("{}", float2);
    println!("{}", float3);
    println!("{}", float4);

    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    let z = 'z';
    let zz = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", z);
    println!("{}", zz);
    println!("{}", heart_eyed_cat);

    print!("(1..5): ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    print!("(1..=5).rev: ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    let sum: i32 = (1..=5).sum();
    println!("1 + 2 + 3 + 4 + 5 = {}", sum);
}
