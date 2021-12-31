// Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.
// Example 1:

// Input: expression = "2-1-1"
// Output: [0,2]
// Explanation:
// ((2-1)-1) = 0
// (2-(1-1)) = 2

use std::collections::HashMap;

fn solve(input: String) -> Vec<i32> {
    let mut cache: HashMap<String, Vec<i32>> = HashMap::new();

    solve_with_cache(input, &mut cache)
}

fn solve_with_cache(input: String, cache: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
    if cache.contains_key(&input) {
        return cache[&input].clone();
    }

    if input.parse::<i32>().is_ok() {
        return vec![input.parse().unwrap()];
    }

    let mut result = vec![];
    for (index, symbol) in input.chars().enumerate() {
        if symbol == '-' || symbol == '+' || symbol == '*' || symbol == '/' {
            let prefix = &input[..index];
            let suffix = &input[index + 1..];

            let prefix_results = solve_with_cache(prefix.to_string(), cache);
            let suffix_results = solve_with_cache(suffix.to_string(), cache);

            for prefix_result in &prefix_results {
                for suffix_results in &suffix_results {
                    result.push(execute_operation(symbol, prefix_result, suffix_results));
                }
            }
        }
    }

    cache.insert(input, result.clone());

    result
}

fn execute_operation(symbol: char, prefix_result: &i32, suffix_results: &i32) -> i32 {
    if symbol == '-' {
        return prefix_result - suffix_results;
    }

    if symbol == '*' {
        return prefix_result * suffix_results;
    }

    if symbol == '/' {
        return prefix_result / suffix_results;
    }

    prefix_result + suffix_results
}

fn main() {
    let input = String::from("5*4-3*2");
    let result = solve(input);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_correct() {
        let input = String::from("5*4-3");
        let result = solve(input);

        assert_eq!(result, [5, 17]);
    }

    #[test]
    fn when_single_number_should_return_number() {
        let input = String::from("5");
        let result = solve(input);

        assert_eq!(result, [5]);
    }

    #[test]
    fn when_double_digit_number_should_return_number() {
        let input = String::from("51");
        let result = solve(input);

        assert_eq!(result, [51]);
    }

    #[test]
    fn when_4_numbers_should_return_correct() {
        let input = String::from("2*3-4*5");
        let result = solve(input);

        assert_eq!(result, [-34, -10, -14, -10, 10]);
    }
}
