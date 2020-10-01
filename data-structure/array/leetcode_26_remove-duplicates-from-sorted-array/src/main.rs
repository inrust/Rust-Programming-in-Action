struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                if j - i > 1 {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
        }

        (i + 1) as i32
    }
}

fn main() {
    let mut vec: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
    let len = Solution::remove_duplicates(&mut vec);
    for i in 0..len as usize {
        print!("{} ", vec[i]);
    }
}
