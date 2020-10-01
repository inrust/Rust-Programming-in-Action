use async_std::task;
use sort_middleware_lib::async_lib;

fn main() {
    let n = 20000;
    let start = time::get_time();

    let arr1 = random_data_lib::generate_random_array(n, 0, n);
    let arr2 = arr1.clone();
    let arr3 = arr1.clone();
    let arr4 = arr1.clone();
    let arr5 = arr1.clone();
    let arr6 = arr1.clone();

    let arr11 = random_data_lib::generate_nearly_ordered_array(n, 100);
    let arr12 = arr11.clone();
    let arr13 = arr11.clone();
    let arr14 = arr11.clone();
    let arr15 = arr11.clone();
    let arr16 = arr11.clone();

    let bubble_random_sort_task = task::spawn(async {
        async_lib::bubble_sort(arr1, "bubble_random_sort").await;
    });
    let selection_random_sort_task = task::spawn(async {
        async_lib::selection_sort(arr2, "selection_random_sort").await;
    });
    let insertion_random_sort_task = task::spawn(async {
        async_lib::insertion_sort(arr3, "insertion_random_sort").await;
    });
    let heap_random_sort_task = task::spawn(async {
        async_lib::heap_sort(arr4, "heap_random_sort").await;
    });
    let merge_random_sort_task = task::spawn(async {
        async_lib::merge_sort(arr5, "merge_random_sort").await;
    });
    let quick_random_sort_task = task::spawn(async {
        async_lib::quick_sort(arr6, "quick_random_sort").await;
    });

    let bubble_nearly_ordered_sort_task = task::spawn(async {
        async_lib::bubble_sort(arr11, "bubble_nearly_ordered_sort").await;
    });
    let selection_nearly_ordered_sort_task = task::spawn(async {
        async_lib::selection_sort(arr12, "selection_nearly_ordered_sort").await;
    });
    let insertion_nearly_ordered_sort_task = task::spawn(async {
        async_lib::insertion_sort(arr13, "insertion_nearly_ordered_sort").await;
    });
    let heap_nearly_ordered_sort_task = task::spawn(async {
        async_lib::heap_sort(arr14, "heap_nearly_ordered_sort").await;
    });
    let merge_nearly_ordered_sort_task = task::spawn(async {
        async_lib::merge_sort(arr15, "merge_nearly_ordered_sort").await;
    });
    let quick_nearly_ordered_sort_task = task::spawn(async {
        async_lib::quick_sort(arr16, "quick_nearly_ordered_sort").await;
    });

    task::block_on(bubble_random_sort_task);
    task::block_on(selection_random_sort_task);
    task::block_on(insertion_random_sort_task);
    task::block_on(heap_random_sort_task);
    task::block_on(merge_random_sort_task);
    task::block_on(quick_random_sort_task);

    task::block_on(bubble_nearly_ordered_sort_task);
    task::block_on(selection_nearly_ordered_sort_task);
    task::block_on(insertion_nearly_ordered_sort_task);
    task::block_on(heap_nearly_ordered_sort_task);
    task::block_on(merge_nearly_ordered_sort_task);
    task::block_on(quick_nearly_ordered_sort_task);

    let end = time::get_time();
    println!("duration: {:?}", (end - start).num_milliseconds());
}
