fn binary_search(arr: &[i32], target_value: &i32) -> Option<usize> {
    let array_length = arr.len();
    let mut min_index: i32 = 0;
    let mut max_index: i32 = array_length as i32 - 1;

    while min_index <= max_index {
        let middle_index = ((max_index - min_index) / 2) + min_index;
        let middle_index_as_usize = middle_index as usize;
        let middle_value = &arr[middle_index_as_usize];

        if middle_value == target_value {
            return Some(middle_index_as_usize);
        }

        if middle_value < target_value {
            min_index = middle_index + 1;
        }

        if middle_value > target_value {
            max_index = middle_index - 1;
        }
    }
    None
}

fn log_binary_search_result(arr: &[i32], target_value: &i32) {
    if let Some(result) = binary_search(arr, target_value) {
        println!("Found {} at index {} of the array.", target_value, result);
    } else {
        println!("{} was not found in the array.", target_value);
    }
}

fn main() {
    let arr: [i32; 20] = [
        1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210, 222,
    ];
    let target: i32 = 88;
    log_binary_search_result(&arr, &target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_value_is_found() {
        let valid_arr = [
            1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210,
            222,
        ];
        for i in 0..valid_arr.len() {
            assert_eq!(i, binary_search(&valid_arr, &valid_arr[i]).unwrap());
        }
    }

    #[test]
    fn assert_value_is_not_found() {
        let sorted_arr = [
            1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210,
            222,
        ];
        let unsorted_arr = [
            2, 22, 48, 58, 61, 73, 84, 90, 100, 119, 132, 154, 160, 177, 187, 197, 201, 211, 2242,
        ];
        for i in unsorted_arr {
            assert_eq!(None, binary_search(&sorted_arr, &i));
        }
    }
}
