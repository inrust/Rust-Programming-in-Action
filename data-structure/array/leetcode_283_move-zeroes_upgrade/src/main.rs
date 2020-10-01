struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }

        for k in i..nums.len() {
            nums[k] = 0;
        }
    }
}

fn main() {
    let mut vec: Vec<i32> = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut vec);
    println!("{:?}", vec);
}
