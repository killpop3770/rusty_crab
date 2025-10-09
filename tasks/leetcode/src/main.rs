use std::vec;

use crate::{
    best_time_to_buy_and_sell_stock::max_profit, kth_largest_element_in_a_stream::KthLargest,
    search_2d_matrix::search_matrix, valid_palindrome::Solution,
};

mod best_time_to_buy_and_sell_stock;
mod contains_duplicate;
pub mod kth_largest_element_in_a_stream;
pub mod search_2d_matrix;
mod valid_palindrome;

fn main() {
    // println!("Hello, world!");
    // let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    // let target = 3;
    // let result = search_matrix(matrix, target);
    // println!("Result: {}", result);

    // let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    // let target = 13;
    // let result = search_matrix(matrix, target);
    // println!("Result: {}", result);

    // let matrix = vec![vec![1]];
    // let target = 1;
    // let result = search_matrix(matrix, target);
    // println!("Result: {}", result);

    // let mut result: Vec<i32> = vec![];
    // let nums = vec![4, 5, 8, 2];
    // let k = 3;
    // let mut kth_largest = KthLargest::new(k, nums);
    // println!("kth_largest: {:?}", kth_largest.vec);
    // let el_to_push = vec![3, 5, 10, 9, 4];
    // for el in el_to_push {
    //     println!("el: {:?}", el);
    //     result.push(kth_largest.add(el));
    //     println!("kth_largest: {:?}", kth_largest.vec);
    // }
    // assert_eq!(vec![4, 5, 5, 8, 8], result);

    // let mut result: Vec<i32> = vec![];
    // let nums = vec![7, 7, 7, 7, 8, 3];
    // let k = 4;
    // let mut kth_largest = KthLargest::new(k, nums);
    // println!("kth_largest: {:?}", kth_largest.vec);
    // let el_to_push = vec![2, 10, 9, 9];
    // for el in el_to_push {
    //     println!("el: {:?}", el);
    //     result.push(kth_largest.add(el));
    //     println!("kth_largest: {:?}", kth_largest.vec);
    // }
    // assert_eq!(vec![7, 7, 7, 8], result);

    // let mut result: Vec<i32> = vec![];
    // let nums = vec![0];
    // let k = 2;
    // let mut kth_largest = KthLargest::new(k, nums);
    // println!("kth_largest: {:?}", kth_largest.vec);
    // let el_to_push = vec![-1, 1, -2, -4, 3];
    // for el in el_to_push {
    //     println!("el: {:?}", el);
    //     result.push(kth_largest.add(el));
    //     println!("kth_largest: {:?}", kth_largest.vec);
    // }
    // assert_eq!(vec![-1, 0, 0, 0, 1], result);

    // let prices = vec![7, 1, 5, 3, 6, 4];
    // let result = max_profit(prices);
    // println!("result {}", result);

    // let prices = vec![7, 6, 4, 3, 1];
    // let result = max_profit(prices);
    // println!("result {}", result);

    // let prices = vec![9, 1, 200, 4, 3, 1];
    // let result = max_profit(prices);
    // println!("result {}", result);

    let pal = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome(pal);
    println!("result {}", result);
}
