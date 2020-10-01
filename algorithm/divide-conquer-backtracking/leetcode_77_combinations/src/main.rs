struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if n < k {
            return Vec::new();
        }

        let mut vecs: Vec<Vec<i32>> = Vec::new();
        let mut vec: Vec<i32> = Vec::new();
        backtrack(&mut vecs, &mut vec, n, k, 1);
        return vecs;
    }
}

fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, n: i32, k: i32, start: usize) {
    if vec.len() == k as usize {
        vecs.push(vec.clone());
        return;
    }

    let mut i = start;
    while i <= (n - (k - vec.len() as i32) + 1) as usize {
        vec.push(i as i32);
        backtrack(vecs, vec, n, k, i + 1);
        vec.remove(vec.len() - 1);

        i += 1;
    }
}

fn main() {
    println!("{:?}", Solution::combine(4, 2));
}
