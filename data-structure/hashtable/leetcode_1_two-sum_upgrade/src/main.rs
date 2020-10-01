use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if map.contains_key(&complement) {
                return vec![i as i32, map[&complement] as i32];
            }

            map.insert(nums[i], i);
        }

        return vec![];
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    println!("{:?}", Solution::two_sum(nums, target));
}
