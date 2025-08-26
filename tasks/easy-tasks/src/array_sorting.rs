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
