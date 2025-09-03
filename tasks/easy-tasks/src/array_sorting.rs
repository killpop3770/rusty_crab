pub fn bubble_sort(array: &mut [i32]) {
    let array_length = array.len();

    for i in 0..array_length - 1 {
        for j in 0..array_length - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

pub fn merge_sort(array: &mut [i32], min_index: usize, max_index: usize) {
    if max_index - min_index > 1 {
        let mid_index = (min_index + max_index) / 2;
        merge_sort(array, min_index, mid_index); //check left half
        merge_sort(array, mid_index, max_index); //check right half
        merge(array, min_index, mid_index, max_index);
    }
}

fn merge(array: &mut [i32], min_index: usize, mid_index: usize, max_index: usize) {
    let mut temp_array = Vec::with_capacity(max_index - min_index);

    let mut left_ptr = min_index;
    let mut right_ptr = mid_index;

    while left_ptr < mid_index && right_ptr < max_index {
        if array[left_ptr] <= array[right_ptr] {
            temp_array.push(array[left_ptr]);
            left_ptr += 1;
        } else {
            temp_array.push(array[right_ptr]);
            right_ptr += 1;
        }
    }

    while left_ptr < mid_index {
        temp_array.push(array[left_ptr]);
        left_ptr += 1;
    }

    while right_ptr < max_index {
        temp_array.push(array[right_ptr]);
        right_ptr += 1;
    }

    for (i, &value) in temp_array.iter().enumerate() {
        array[min_index + i] = value
    }
}
