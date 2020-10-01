use std::cmp;

pub async fn bubble_sort<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>) {
    if arr.is_empty() { return; }

    for i in 0..arr.len() - 1 {
        let mut flag = false;
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;

                flag = true;
            }
        }

        if !flag { break; }
    }
}

pub async fn selection_sort<T: cmp::PartialOrd>(arr: &mut Vec<T>) {
    if arr.is_empty() { return; }

    for i in 0..arr.len() - 1 {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

pub async fn insertion_sort<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>) {
    if arr.is_empty() { return; }

    for i in 1..arr.len() {
        let current = arr[i];

        let mut j = (i - 1) as i32;
        while j >= 0 {
            if arr[j as usize] > current {
                arr[(j + 1) as usize] = arr[j as usize];
            } else {
                break;
            }
            j -= 1;
        }

        arr[(j + 1) as usize] = current;
    }
}

pub async fn heap_sort<T: cmp::PartialOrd>(arr: &mut Vec<T>) {
    build_heap(arr);

    for i in (0..arr.len()).rev() {
        arr.swap(0, i);
        heapify(arr, 0, i);
    }
}


fn build_heap<T: cmp::PartialOrd>(arr: &mut Vec<T>) {
    let len = arr.len();
    for i in (0..len / 2).rev() {
        heapify(arr, i, len);
    }
}

fn heapify<T: cmp::PartialOrd>(arr: &mut Vec<T>, idx: usize, len: usize) {
    let mut idx = idx;
    loop {
        let mut max_pos = idx;
        if 2 * idx + 1 < len && arr[idx] < arr[2 * idx + 1] { max_pos = 2 * idx + 1; }
        if 2 * idx + 2 < len && arr[max_pos] < arr[2 * idx + 2] { max_pos = 2 * idx + 2; }

        if max_pos == idx { break; }
        arr.swap(idx, max_pos);
        idx = max_pos;
    }
}

pub async fn merge_sort<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>) {
    if arr.is_empty() { return; }

    let n = arr.len() - 1;
    merge_sort_recursion(arr, 0, n);
}

fn merge_sort_recursion<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>, left: usize, right: usize) {
    if left >= right { return; }
    let middle = left + (right - left) / 2;

    merge_sort_recursion(arr, left, middle);
    merge_sort_recursion(arr, middle + 1, right);

    merge(arr, left, middle, right);
}

fn merge<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>, left: usize, middle: usize, right: usize) {
    let mut i = left;
    let mut j = middle + 1;
    let mut k = left;
    let mut tmp = vec![];

    while k <= right {
        if i > middle {
            tmp.push(arr[j]);
            j += 1;
            k += 1;
        } else if j > right {
            tmp.push(arr[i]);
            i += 1;
            k += 1;
        } else if arr[i] < arr[j] {
            tmp.push(arr[i]);
            i += 1;
            k += 1;
        } else {
            tmp.push(arr[j]);
            j += 1;
            k += 1;
        }
    }

    for i in 0..=(right - left) {
        arr[left + i] = tmp[i];
    }
}

pub async fn quick_sort<T: cmp::PartialOrd>(arr: &mut Vec<T>) {
    if arr.is_empty() { return; }

    let len = arr.len();
    quick_sort_recursion(arr, 0, len - 1);
}

fn quick_sort_recursion<T: cmp::PartialOrd>(arr: &mut Vec<T>, left: usize, right: usize) {
    if left >= right { return; }

    let pivot = partition(arr, left, right);
    if pivot != 0 {
        quick_sort_recursion(arr, left, pivot - 1);
    }
    quick_sort_recursion(arr, pivot + 1, right);
}

fn partition<T: cmp::PartialOrd>(arr: &mut Vec<T>, left: usize, right: usize) -> usize {
    let pivot = right;
    let mut i = left;

    for j in left..right {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, right);
    i
}
