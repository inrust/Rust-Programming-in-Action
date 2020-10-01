struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        if nums[right] > nums[0] {
            return nums[0];
        }

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[mid + 1] {
                return nums[mid + 1];
            }

            if nums[mid - 1] > nums[mid] {
                return nums[mid];
            }

            if nums[mid] > nums[0] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return -1;
    }
}

fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    println!("{}", Solution::find_min(nums));
}
