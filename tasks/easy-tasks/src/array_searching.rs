use std::cmp::Ordering;

pub fn binary_search(array: &[i32], target: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut min_id = 0;
    let mut max_id = array.len() - 1;

    while min_id <= max_id {
        let current_id = (min_id + max_id) / 2;
        match target.cmp(&array[current_id]) {
            Ordering::Less => match current_id.checked_sub(1) {
                Some(id) => max_id = id,
                None => break,
            },
            Ordering::Equal => {
                return Some(current_id);
            }
            Ordering::Greater => {
                min_id = current_id + 1;
            }
        }
    }

    return None;
}
