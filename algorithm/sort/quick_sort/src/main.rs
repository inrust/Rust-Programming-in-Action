pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return nums; }

    let len = nums.len();
    quick_sort_recursion(&mut nums, 0, len - 1);
    nums
}

fn quick_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right { return; }

    let pivot = partition(nums, left, right);
    if pivot != 0 {
        quick_sort_recursion(nums, left, pivot - 1);
    }
    quick_sort_recursion(nums, pivot + 1, right);
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = right;
    let mut i = left;

    for j in left..right {
        if nums[j] < nums[pivot] {
            nums.swap(i, j);
            i += 1;
        }
    }

    nums.swap(i, right);
    println!("{:?}", nums);
    i
}

fn main() {
    let nums = vec![7, 9, 12, 11, 6, 3];
    quick_sort(nums);
}
