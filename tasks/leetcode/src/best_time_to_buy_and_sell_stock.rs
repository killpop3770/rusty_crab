pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut result_diff = 0_i32;
    let mut min_pointer = 0;
    let mut max_pointer = 0;

    for _ in prices.iter() {
        let current_diff = prices[max_pointer] - prices[min_pointer];

        if current_diff > result_diff {
            result_diff = current_diff;
        }

        if prices[max_pointer] < prices[min_pointer] {
            min_pointer = max_pointer;
        }

        max_pointer += 1;
    }

    return result_diff;
}

// diff = 6
// [7, 1, 5, 3, 6, 4]
//     i
//              j
