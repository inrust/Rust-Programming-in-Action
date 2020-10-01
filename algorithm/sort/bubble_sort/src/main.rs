fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return vec![]; }

    for i in 0..nums.len() - 1 {
        let mut flag = false;
        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;

                flag = true;
            }
        }

        println!("{:?}", nums);
        if !flag { break; }
    }
    nums
}

fn main() {
    let nums = vec![7, 9, 12, 11, 6, 3];
    bubble_sort(nums);
}