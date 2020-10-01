struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return Vec::new();
        }

        let mut vecs: Vec<Vec<i32>> = Vec::new();
        let mut vec: Vec<i32> = Vec::new();
        backtrack(&mut vecs, &mut vec, &nums, 0);
        return vecs;
    }
}

fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
    vecs.push(vec.clone());
    for i in start..nums.len() {
        vec.push(nums[i]);
        backtrack(vecs, vec, &nums, i + 1);
        vec.remove(vec.len() - 1);
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", Solution::subsets(nums));
}
