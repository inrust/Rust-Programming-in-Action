struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n;
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        return fast_pow(x, n);
    }
}

fn fast_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let half = fast_pow(x, n / 2);
    return if n % 2 == 0 {
        half * half
    } else {
        half * half * x
    }
}

fn main() {
    let x = 2.0;
    let n = -2;

    println!("{}", Solution::my_pow(x, n));
}
