use std::collections::HashMap;

use crate::{
    array_searching::{binary_search, search_insert_position},
    array_sorting::{bubble_sort, merge_sort},
    common_serde::{Deserialize, MyStruct, Serialize, Serializer},
};

mod array_searching;
pub mod array_sorting;
pub mod common_hash_map;
mod common_serde;
mod common_vec;

fn main() {
    println!("Hello, world!");

    // let mut test_array = [42, 100, 612, 318, 0, 11, 234, 15];
    // println!("before bubble_sort: {:?}", test_array);
    // bubble_sort(&mut test_array);
    // println!("after bubble_sort: {:?}", test_array);

    // let target = -3180;
    // let result = binary_search(&test_array, target);
    // match result {
    //     Some(res) => println!("result binary search: {}", res),
    //     None => println!("not found target in array"),
    // }

    // let mut test_array = [100, -100];
    // println!("before sort: {:?}", test_array);
    // let array_len = test_array.len();
    // merge_sort(&mut test_array, 0, array_len);
    // println!("after sort: {:?}", test_array);

    // let test_array = [0, 15, 42, 100, 612];
    // println!("test_array: {:?}", test_array);
    // let target = 43;
    // let result = search_insert_position(&test_array, target);
    // println!("result search_insert_position: {}", result)

    let my_struct = MyStruct {
        value_i32: 2147483647,
        value_string: String::from("Hello, friend!"),
    };

    let mut serializer_buffer = Vec::<u8>::new();
    if let Err(error) = my_struct.serialize(&mut serializer_buffer) {
        println!("Some error occurred with serialization: {:?} !", error);
    }
    println!("Serialized data: {:?}", serializer_buffer.clone().finish());

    let mut deserializer_buffer = serializer_buffer.as_slice();
    match MyStruct::deserialize(&mut deserializer_buffer) {
        Ok(data) => println!("Deserialized data: {:?}", data),
        Err(error) => println!("Some error occurred with serialization: {:?} !", error),
    }
}
