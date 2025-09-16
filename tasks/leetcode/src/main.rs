use std::vec;

use crate::search_2d_matrix::search_matrix;

pub mod search_2d_matrix;

fn main() {
    println!("Hello, world!");
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    let result = search_matrix(matrix, target);
    println!("Result: {}", result);

    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 13;
    let result = search_matrix(matrix, target);
    println!("Result: {}", result);

    let matrix = vec![vec![1]];
    let target = 1;
    let result = search_matrix(matrix, target);
    println!("Result: {}", result);
}
