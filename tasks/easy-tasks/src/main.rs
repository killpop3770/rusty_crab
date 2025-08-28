use crate::{array_searching::binary_search, array_sorting::bubble_sort};

mod array_searching;
pub mod array_sorting;

fn main() {
    println!("Hello, world!");

    let mut test_array = [42, 100, 612, 318, 0, 11, 234, 15];
    println!("before bubble_sort: {:?}", test_array);
    bubble_sort(&mut test_array);
    println!("after bubble_sort: {:?}", test_array);

    let target = -3180;
    let result = binary_search(&test_array, target);
    match result {
        Some(res) => println!("result binary search: {}", res),
        None => println!("not found target in array"),
    }
}
