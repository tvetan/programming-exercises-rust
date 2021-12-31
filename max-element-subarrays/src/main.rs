// Given an array of integers and a number k, where 1 <= k <= length of the array,
// compute the maximum values of each subarray of length k.
// For example, given array = [10, 5, 2, 7, 8, 7] and k = 3,
// we should get: [10, 7, 8, 8], since:

// 10 = max(10, 5, 2)
// 7 = max(5, 2, 7)
// 8 = max(2, 7, 8)
// 8 = max(7, 8, 7)

use std::collections::VecDeque;

fn get_all_maximums(items: Vec<i32>, k: usize) -> Vec<i32> {
    let mut result = vec![];
    let mut valid_items_indexes = VecDeque::new();

    for index in 0..k {
        remove_items_smaller_then_current(&mut valid_items_indexes, &items, index);

        valid_items_indexes.push_back(index);
    }

    for index in k..items.len() {
        let max = valid_items_indexes.front().unwrap();
        result.push(items[*max]);

        remove_items_not_in_subarray(&mut valid_items_indexes, index, k);
        remove_items_smaller_then_current(&mut valid_items_indexes, &items, index);

        valid_items_indexes.push_back(index);
    }

    result.push(items[valid_items_indexes.pop_front().unwrap()]);

    result
}

fn remove_items_not_in_subarray(valid_items_index: &mut VecDeque<usize>, index: usize, k: usize) {
    while valid_items_index.is_empty() == false && *valid_items_index.front().unwrap() <= index - k
    {
        valid_items_index.pop_front();
    }
}

fn remove_items_smaller_then_current(
    valid_items_index: &mut VecDeque<usize>,
    items: &Vec<i32>,
    index: usize,
) {
    while valid_items_index.is_empty() == false
        && items[index] > items[*valid_items_index.back().unwrap()]
    {
        valid_items_index.pop_back();
    }
}

fn main() {
    let items = vec![10, 12, 8, 11, 18];
    let k = 3;

    let result = get_all_maximums(items, k);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_correct() {
        let items = vec![10, 12, 8, 11, 11];
        let k = 3;

        let result = get_all_maximums(items, k);

        assert_eq!(result, [12, 12, 11]);
    }

    #[test]
    fn should_return_correct_when_first_element_is_max() {
        let items = vec![14, 12, 8, 11, 11];
        let k = 3;

        let result = get_all_maximums(items, k);

        assert_eq!(result, [14, 12, 11]);
    }

    #[test]
    fn should_return_correct_when_last_element_is_max() {
        let items = vec![14, 12, 8, 11, 17];
        let k = 3;

        let result = get_all_maximums(items, k);

        assert_eq!(result, [14, 12, 17]);
    }

    #[test]
    fn should_return_correct_when_same_elements() {
        let items = vec![14, 14, 14, 14, 14];
        let k = 3;

        let result = get_all_maximums(items, k);

        assert_eq!(result, [14, 14, 14]);
    }

    #[test]
    fn should_return_correct_when_negative_elements() {
        let items = vec![1, -14, 3, 7, -1];
        let k = 3;

        let result = get_all_maximums(items, k);

        assert_eq!(result, [3, 7, 7]);
    }

    #[test]
    fn should_return_correct_when_n_equal_k() {
        let items = vec![1, 1, 3];
        let k = 3;

        let result = get_all_maximums(items, k);

        assert_eq!(result, [3]);
    }
}
