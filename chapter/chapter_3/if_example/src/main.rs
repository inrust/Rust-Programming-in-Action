fn if_simple(age: i32) {
    if age > 18 {
        println!("You are an adult.");
    }
}

fn if_else(age: i32) {
    if age > 18 {
        println!("You are an adult.");
    } else {
        println!("You are not an adult");
    }
}

fn if_elseif_else(age: i32) {
    if age < 1 {
        println!("You are a baby.");
    } else if age < 3 {
        println!("You are a toddler.");
    } else if age < 5 {
        println!("You are a preschooler.");
    } else if age < 10 {
        println!("You are a schoolchild.");
    } else if age < 12 {
        println!("You are a preteen.");
    } else if age < 18 {
        println!("You are a teenager.");
    } else {
        println!("You are an adult.");
    }
}

fn main() {
    if_simple(20);
    if_else(15);
    if_elseif_else(15);
}
