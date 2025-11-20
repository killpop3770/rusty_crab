use std::{collections::BinaryHeap, vec};

use crate::{
    best_time_to_buy_and_sell_stock::max_profit,
    climbing_stairs::{climb_stairs, climb_stairs_with_tree, visualize_heap},
    kth_largest_element_in_a_stream::KthLargest,
    search_2d_matrix::search_matrix,
    valid_palindrome::Solution,
    valid_parentheses::Solution as Solution2,
};

mod best_time_to_buy_and_sell_stock;
pub mod climbing_stairs;
pub(crate) mod coin_change;
mod contains_duplicate;
pub mod kth_largest_element_in_a_stream;
pub mod search_2d_matrix;
mod valid_palindrome;
pub mod valid_parentheses;

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

    // let pal = "A man, a plan, a canal: Panama".to_string();
    // let result = Solution::is_palindrome(pal);
    // println!("result {}", result);

    // let s = "()[]{}".to_string();
    // let s = "()[{]{}".to_string();
    // let s = "([)]".to_string();
    // let s = "([])".to_string();
    // let result = Solution2::is_valid(s);
    // println!("result {:?}", result);

    // let stairs = 4;
    // let result = climb_stairs(stairs);
    // println!("result {:?}", result);

    // let mut binary_tree = BinaryHeap::new();
    // binary_tree.push(5);
    // binary_tree.push(3);
    // binary_tree.push(7);
    // binary_tree.push(1);
    // visualize_heap(&binary_tree);
    // println!()

    // climb_stairs_with_tree(4);
    let coins = vec![186, 419, 83, 408];
    let target = 6249;
    // let coins = vec![1, 2];
    // let target = 100;
    let count = coin_change::coin_change_r(coins, target);
    println!("{count}");
}
