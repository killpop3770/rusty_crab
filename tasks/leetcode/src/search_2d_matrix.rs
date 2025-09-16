// "struct: {
//     some_field: 42,
// }"

use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() {
        return false;
    }

    let mut min_value = 0;
    let mut max_value = 0;

    for row in matrix {
        min_value = row[0];
        max_value = row[row.len() - 1];

        if target < min_value {
            // if target = 10 in [11, ...],
            return false;
        }

        if target <= max_value {
            // if target = 10 in [..., 20],
            match binary_search(&row, target) {
                Some(_) => return true,
                None => return false,
            };
        } else {
            // next row
            continue;
        }
    }

    return false;
}

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
