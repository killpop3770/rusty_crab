use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut dict = HashSet::<i32>::new();
        for num in nums {
            if !dict.contains(&num) {
                dict.insert(num);
            } else {
                return true;
            }
        }
        return false;

        // !nums.into_iter().all(|f| dict.insert(f))
    }
}
