struct Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }

        let len = nums.len();
        let mut tmp = vec![0; len];
        return sort_count(&mut nums, &mut tmp, 0, len - 1) as i32;
    }
}

fn sort_count(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, left: usize, right: usize) -> usize {
    if left >= right { return 0; }
    let middle = left + (right - left) / 2;

    let mut count = sort_count(nums, tmp, left, middle) + sort_count(nums, tmp, middle + 1, right);

    let mut point_left = left;
    let mut point_right = middle + 1;
    while point_left <= middle && point_right <= right {
        if nums[point_left] as i64 > 2 * nums[point_right] as i64 {
            count += middle - point_left + 1;
            point_right += 1;
        } else {
            point_left += 1;
        }
    }

    merge(nums, tmp, left, middle, right);
    count
}

fn merge(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let mut index = 0;
    let mut point_left = left;
    let mut point_right = middle + 1;
    while point_left <= middle && point_right <= right {
        if nums[point_left] <= nums[point_right] {
            tmp[index] = nums[point_left];
            index += 1;
            point_left += 1;
        } else {
            tmp[index] = nums[point_right];
            index += 1;
            point_right += 1
        }
    }

    while point_left <= middle {
        tmp[index] = nums[point_left];
        index += 1;
        point_left += 1;
    }

    while point_right <= right {
        tmp[index] = nums[point_right];
        index += 1;
        point_right += 1
    }

    for i in left..=right {
        nums[i] = tmp[i - left];
    }
}

fn main() {
    let nums = vec![2, 4, 3, 5, 1];
    println!("{}", Solution::reverse_pairs(nums));
}
