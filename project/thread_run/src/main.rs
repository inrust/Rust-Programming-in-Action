use threadpool::ThreadPool;
use sort_middleware_lib::sync_lib;

fn main() {
    let n = 20000;
    let n_workers = 5;
    let pool = ThreadPool::new(n_workers);
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

    pool.execute(move || {
        sync_lib::bubble_sort(arr1, "bubble_random_sort");
    });

    pool.execute(move || {
        sync_lib::selection_sort(arr2, "selection_random_sort");
    });

    pool.execute(move || {
        sync_lib::insertion_sort(arr3, "insertion_random_sort");
    });

    pool.execute(move || {
        sync_lib::heap_sort(arr4, "heap_random_sort");
    });

    pool.execute(move || {
        sync_lib::merge_sort(arr5, "merge_random_sort");
    });

    pool.execute(move || {
        sync_lib::quick_sort(arr6, "quick_random_sort");
    });

    pool.execute(move || {
        sync_lib::bubble_sort(arr11, "bubble_nearly_ordered_sort");
    });

    pool.execute(move || {
        sync_lib::selection_sort(arr12, "selection_nearly_ordered_sort");
    });

    pool.execute(move || {
        sync_lib::insertion_sort(arr13, "insertion_nearly_ordered_sort");
    });

    pool.execute(move || {
        sync_lib::heap_sort(arr14, "heap_nearly_ordered_sort");
    });

    pool.execute(move || {
        sync_lib::merge_sort(arr15, "merge_nearly_ordered_sort");
    });

    pool.execute(move || {
        sync_lib::quick_sort(arr16, "quick_nearly_ordered_sort");
    });

    pool.join();

    let end = time::get_time();
    println!("duration: {:?}", (end - start).num_milliseconds());
}
