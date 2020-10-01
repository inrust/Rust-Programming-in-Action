fn loop_counter() {
    let mut count = 0;
    let counter = loop {
        count += 1;
        let counter = count * 2;
        println!("count: {}, counter: {}", count, counter);

        if count == 10 {
            break counter;
        }
    };
}

fn while_counter() {
    let mut count = 0;
    let mut counter = 0;
    while count != 10 {
        count += 1;
        counter = count * 2;
        println!("count: {}, counter: {}", count, counter);
    }
}

fn for_counter() {
    let mut counter = 0;
    for count in 1..=10 {
        counter = count * 2;
        println!("count: {}, counter: {}", count, counter);
    }
}

fn continue_break() {
    for i in 0..10 {
        if i == 0 || i == 4 { continue; }
        if i == 6 { break; }

        println!("i: {}", i);
    };
}

fn main() {
    loop_counter();
    while_counter();
    for_counter();
    continue_break();
}
