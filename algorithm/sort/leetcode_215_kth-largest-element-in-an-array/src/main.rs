use rand::Rng;

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() || k > nums.len() as i32 { return -1; }
        let len = nums.len();
        return quick_select(&mut nums, 0, len - 1, len - k as usize);
    }
}

fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k_smallest: usize) -> i32 {
    if left == right { return nums[left]; }

    let mut rng = rand::thread_rng();
    let mut pivot_index = left + rng.gen_range(0, right - left);
    pivot_index = partition(nums, left, right, pivot_index);

    return if k_smallest == pivot_index {
        nums[k_smallest]
    } else if k_smallest < pivot_index {
        quick_select(nums, left, pivot_index - 1, k_smallest)
    } else {
        quick_select(nums, pivot_index + 1, right, k_smallest)
    };
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot = nums[pivot_index];
    nums.swap(pivot_index, right);
    let mut store_index = left;

    for i in left..right {
        if nums[i] < pivot {
            nums.swap(store_index, i);
            store_index += 1;
        }
    }

    nums.swap(store_index, right);
    store_index
}

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    println!("{}", Solution::find_kth_largest(nums, 2));
}
