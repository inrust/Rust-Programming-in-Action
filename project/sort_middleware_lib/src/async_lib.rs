use sort_lib::async_lib;

pub async fn bubble_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::get_time();
    async_lib::bubble_sort(&mut arr).await;
    let end = time::get_time();
    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}

pub async fn selection_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::get_time();
    async_lib::selection_sort(&mut arr).await;
    let end = time::get_time();
    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}

pub async fn insertion_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::get_time();
    async_lib::insertion_sort(&mut arr).await;
    let end = time::get_time();
    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}

pub async fn heap_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::get_time();
    async_lib::heap_sort(&mut arr).await;
    let end = time::get_time();
    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}

pub async fn merge_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::get_time();
    async_lib::merge_sort(&mut arr).await;
    let end = time::get_time();
    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}

pub async fn quick_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::get_time();
    async_lib::quick_sort(&mut arr).await;
    let end = time::get_time();
    println!("{} duration: {:?}", function_name, (end - start).num_milliseconds());
}
