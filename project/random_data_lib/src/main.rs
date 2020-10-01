use random_data_lib;

fn main() {
    let n = 20;
    let arr1 = random_data_lib::generate_random_array(n, 0, n);
    let arr2 = random_data_lib::generate_nearly_ordered_array(n, 1);

    println!("{:?}", arr1);
    println!("{:?}", arr2);
}