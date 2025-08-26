use std::cmp::Ordering;

pub fn binary_search(array: &[i32], target: i32) -> usize {
    let mut min_id = 0;
    let mut max_id = array.len() - 1;

    if array.len() == 0 || array.len() == 1 {
        return 0;
    }

    while min_id <= max_id {
        let current_id = (min_id + max_id) / 2;
        match target.cmp(&array[current_id]) {
            Ordering::Less => {
                max_id = current_id - 1;
            }
            Ordering::Equal => {
                return current_id;
            }
            Ordering::Greater => {
                min_id = current_id + 1;
            }
        }
    }

    return 0;
}
