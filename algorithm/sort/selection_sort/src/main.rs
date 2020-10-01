fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return vec![]; }

    for i in 0..nums.len() - 1 {
        let mut min_index = i;

        for j in i + 1..nums.len() {
            if nums[j] < nums[min_index] {
                min_index = j;
            }
        }

        if i != min_index {
            nums.swap(i, min_index);
        }

        println!("{:?}", nums);
    }

    nums
}

fn main() {
    let nums = vec![7, 9, 12, 11, 6, 3];
    selection_sort(nums);
}
