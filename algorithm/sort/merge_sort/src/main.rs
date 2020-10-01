pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return nums; }

    let n = nums.len() - 1;
    merge_sort_recursion(&mut nums, 0, n);
    nums
}

fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right { return; }
    let middle = left + (right - left) / 2;

    merge_sort_recursion(nums, left, middle);
    merge_sort_recursion(nums, middle + 1, right);

    merge(nums, left, middle, right);
}

fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let mut i = left;
    let mut j = middle + 1;
    let mut k = left;
    let mut tmp = vec![];

    while k <= right {
        if i > middle {
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        } else if j > right {
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else if nums[i] < nums[j] {
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else {
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        }
    }

    for i in 0..=(right - left) {
        nums[left + i] = tmp[i];
    }

    println!("{:?}", nums);
}

fn main() {
    let nums = vec![7, 9, 12, 11, 6, 3];
    merge_sort(nums);
}
