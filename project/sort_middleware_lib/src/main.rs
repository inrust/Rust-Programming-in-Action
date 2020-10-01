use sort_middleware_lib::sync_lib;

fn main() {
    let n = 20000;
    let start = time::get_time();

    let arr1 = random_data_lib::generate_random_array(n, 0, n);
    let arr2 = arr1.clone();
    let arr3 = arr1.clone();
    let arr4 = arr1.clone();
    let arr5 = arr1.clone();
    let arr6 = arr1.clone();

    sync_lib::bubble_sort(arr1, "bubble_random_sort");
    sync_lib::selection_sort(arr2, "selection_random_sort");
    sync_lib::insertion_sort(arr3, "insertion_random_sort");
    sync_lib::heap_sort(arr4, "heap_random_sort");
    sync_lib::merge_sort(arr5, "merge_random_sort");
    sync_lib::quick_sort(arr6, "quick_random_sort");

    let arr1 = random_data_lib::generate_nearly_ordered_array(n, 100);
    let arr2 = arr1.clone();
    let arr3 = arr1.clone();
    let arr4 = arr1.clone();
    let arr5 = arr1.clone();
    let arr6 = arr1.clone();

    sync_lib::bubble_sort(arr1, "bubble_nearly_ordered_sort");
    sync_lib::selection_sort(arr2, "selection_nearly_ordered_sort");
    sync_lib::insertion_sort(arr3, "insertion_nearly_ordered_sort");
    sync_lib::heap_sort(arr4, "heap_nearly_ordered_sort");
    sync_lib::merge_sort(arr5, "merge_nearly_ordered_sort");
    sync_lib::quick_sort(arr6, "quick_nearly_ordered_sort");

    let end = time::get_time();
    println!("duration: {:?}", (end - start).num_milliseconds());
}