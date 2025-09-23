use std::cmp::Ordering;

pub struct KthLargest {
    pub vec: Vec<i32>,
    k: i32,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut result = KthLargest { vec: nums, k };
        result.vec.sort();
        return result;
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.vec.is_empty() {
            self.vec.push(val);
            return self.vec[self.kth_index()];
        }

        if self.vec.len() >= self.k as usize && val > self.vec[self.kth_index()] {
            self.insert_in_searched_position(val);
        } else {
            self.insert_in_searched_position(val);
        }

        return self.vec[self.kth_index()];
    }

    fn kth_index(&self) -> usize {
        if self.vec.len() >= self.k as usize {
            self.vec.len() - self.k as usize
        } else {
            0
        }
    }

    fn insert_in_searched_position(&mut self, val: i32) {
        let index_position = KthLargest::search_insert_position(&self.vec, val);
        match index_position.is_negative() {
            true => self.vec.insert(0, val),
            false => self.vec.insert(index_position as usize, val),
        }
    }

    fn search_insert_position(array: &Vec<i32>, target: i32) -> i32 {
        if array.is_empty() {
            return 0;
        }

        let mut min_id = 0;
        let mut max_id = array.len() - 1;
        let expected_id: i32;

        while min_id <= max_id {
            let current_id = (min_id + max_id) / 2;
            match target.cmp(&array[current_id]) {
                Ordering::Less => match current_id.checked_sub(1) {
                    Some(id) => max_id = id,
                    None => break,
                },
                Ordering::Equal => {
                    return current_id as i32;
                }
                Ordering::Greater => {
                    min_id = current_id + 1;
                }
            }
        }

        match min_id > max_id {
            true => expected_id = min_id as i32,
            false => expected_id = min_id as i32 - 1,
        }

        return expected_id;
    }
}
