use rand::Rng;

pub fn generate_random_array(n: i32, range_left: i32, range_right: i32) -> Vec<i32> {
    let arr = vec![0; n as usize];
    if range_left > range_right {
        return arr;
    }

    let mut rng = rand::thread_rng();
    return arr.iter().map(|_| rng.gen_range(range_left, range_right)).collect();
}

pub fn generate_nearly_ordered_array(n: i32, swap_times: i32) -> Vec<i32> {
    let mut arr = vec![0; n as usize];
    for i in 0..n {
        arr[i as usize] = i;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..swap_times {
        let posx = rng.gen_range(0, n);
        let posy = rng.gen_range(0, n);
        arr.swap(posx as usize, posy as usize);
    }

    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_array_test() {
        let n = 20;
        let arr = generate_random_array(n, 0, n);
        assert_eq!(arr.len(), n as usize);
    }

    #[test]
    fn nearly_ordered_array_test() {
        let n = 20;
        let arr = generate_nearly_ordered_array(n, 1);
        assert_eq!(arr.len(), n as usize);
    }
}
