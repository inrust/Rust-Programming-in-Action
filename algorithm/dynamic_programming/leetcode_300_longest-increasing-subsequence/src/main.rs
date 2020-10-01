struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut dp = vec![1; nums.len()];
        let mut res = 1;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            res = res.max(dp[i]);
        }
        res
    }
}

fn main() {
    let nums = vec![2, 9, 3, 6, 5, 1, 7];
    println!("{}", Solution::length_of_lis(nums));
}
