use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 || k == 1 {
            return nums;
        }

        let mut res: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();
        for i in 0..nums.len() {
            push(&mut deque, nums[i]);

            if (i as i32) > k - 1 {
                pop(&mut deque, nums[i - k as usize]);
                res.push(max(&deque));
            } else if (i as i32) == k - 1 {
                res.push(max(&deque));
            }
        }

        return res;
    }
}

fn push(deque: &mut VecDeque<i32>, n: i32) {
    while !deque.is_empty() && *deque.back().unwrap() < n {
        deque.pop_back();
    }
    deque.push_back(n);
}

fn pop(deque: &mut VecDeque<i32>, n: i32) {
    if !deque.is_empty() && *deque.front().unwrap() == n {
        deque.pop_front();
    }
}

fn max(deque: &VecDeque<i32>) -> i32 {
    return *deque.front().unwrap();
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    println!("{:?}", Solution::max_sliding_window(nums, 3));
}
