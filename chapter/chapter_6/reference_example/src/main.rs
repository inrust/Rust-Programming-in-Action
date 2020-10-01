fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let answer = sum_vec(&vec1, &vec2);
    println!("v1: {:?}, v2: {:?}, sum: {}", vec1, vec2, answer);

    let mut vec = Vec::new();
    push_vec(&mut vec, 1);
    push_vec(&mut vec, 2);
    push_vec(&mut vec, 2);
    push_vec(&mut vec, 5);
    println!("vec: {:?}", vec);

    let mut x = 6;

    let y = &mut x;
    *y += 1;

    println!("x: {}", x);
}

fn push_vec(v: &mut Vec<i32>, value: i32) {
    if v.is_empty() || v.get(v.len() - 1).unwrap() < &value {
        v.push(value);
    }
}

fn sum_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let sum1: i32 = v1.iter().sum();
    let sum2: i32 = v2.iter().sum();

    sum1 + sum2
}
