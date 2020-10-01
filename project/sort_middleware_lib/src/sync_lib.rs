use std::cmp;

use sort_lib::sync_lib;

pub fn bubble_sort(mut arr: Vec<i32>, function_name: &str) {
    sort(sync_lib::bubble_sort, &mut arr, function_name);
}

pub fn selection_sort(mut arr: Vec<i32>, function_name: &str) {
    sort(sync_lib::selection_sort, &mut arr, function_name);
}

pub fn insertion_sort(mut arr: Vec<i32>, function_name: &str) {
    sort(sync_lib::insertion_sort, &mut arr, function_name);
}

pub fn heap_sort(mut arr: Vec<i32>, function_name: &str) {
    sort(sync_lib::heap_sort, &mut arr, function_name);
}

pub fn merge_sort(mut arr: Vec<i32>, function_name: &str) {
    sort(sync_lib::merge_sort, &mut arr, function_name);
}

pub fn quick_sort(mut arr: Vec<i32>, function_name: &str) {
    sort(sync_lib::quick_sort, &mut arr, function_name);
}

fn sort<T: cmp::PartialOrd>(sort_function: fn(&mut Vec<T>), arr: &mut Vec<T>, function_name: &str) {
    let start = time::get_time();
    sort_function(arr);
    let end = time::get_time();

    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}
