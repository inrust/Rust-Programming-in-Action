pub fn heap_sort(nums: &mut Vec<i32>) {
    build_heap(nums);

    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        heapify(nums, 0, i);
        println!("{:?}", nums);
    }
}


fn build_heap(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in (0..len / 2).rev() {
        heapify(nums, i, len);
    }
}

fn heapify(nums: &mut Vec<i32>, idx: usize, len: usize) {
    let mut idx = idx;
    loop {
        let mut max_pos = idx;
        if 2 * idx + 1 < len && nums[idx] < nums[2 * idx + 1] { max_pos = 2 * idx + 1; }
        if 2 * idx + 2 < len && nums[max_pos] < nums[2 * idx + 2] { max_pos = 2 * idx + 2; }

        if max_pos == idx { break; }
        nums.swap(idx, max_pos);
        idx = max_pos;
    }
}

fn main() {
    let mut nums = vec![7, 9, 12, 11, 6, 3];
    heap_sort(&mut nums);
}
