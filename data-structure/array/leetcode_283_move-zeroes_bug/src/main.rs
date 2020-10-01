struct Solution;

impl Solution {
    pub fn move_zeroes(nums: Vec<i32>) {
        let i = 0;
        let j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }

        let k = i;
        while k < nums.len() {
            nums[k] = 0;
            k += 1;
        }
    }
}

fn main() {
    let vec: Vec<i32> = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(vec);
    println!("{:?}", vec);
}
