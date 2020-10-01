use std::cmp;

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = height.len() as i32 - 1;
        while i < j {
            let mut min_height: i32 = 0;
            let x = height[i as usize];
            let y = height[j as usize];
            if x < y {
                min_height = x;
                i += 1;
            } else {
                min_height = y;
                j -= 1;
            }
            max = cmp::max(max, (j - i + 1) * min_height);
        }

        return max;
    }
}

fn main() {
    let vec: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let area = Solution::max_area(vec);
    println!("{:?}", area);
}
