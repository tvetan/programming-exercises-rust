// Given a string s, return the longest palindromic substring in s.
// Example 1:

// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.

fn longest_palindrome(s: String) -> String {
    if s.len() < 2 {
        return s;
    }

    let mut table = vec![vec![false; s.len()]; s.len()];

    let mut longest_palindrome_length = 1;
    let mut result_start_index = 0;
    let mut result_end_index = 0;

    set_single_digit_palindromes(s.len(), &mut table);

    set_double_digit_palindroms(
        &s,
        &mut table,
        &mut longest_palindrome_length,
        &mut result_start_index,
        &mut result_end_index,
    );

    set_complex_palindromes(
        &s,
        &mut table,
        longest_palindrome_length,
        &mut result_start_index,
        &mut result_end_index,
    );

    s[result_start_index..=result_end_index].to_string()
}

fn set_complex_palindromes(
    s: &String,
    table: &mut Vec<Vec<bool>>,
    longest_palindrome_length: usize,
    result_start_index: &mut usize,
    result_end_index: &mut usize,
) {
    let symbols = s.chars().collect::<Vec<char>>();
    for length in 3..=s.len() {
        for start_index in 0..s.len() - length + 1 {
            let end_index = length + start_index - 1;

            let start_symbol = symbols[start_index];
            let end_symbol = symbols[end_index];

            if start_symbol == end_symbol && table[start_index + 1][end_index - 1] {
                table[start_index][end_index] = true;

                if longest_palindrome_length < length {
                    *result_start_index = start_index;
                    *result_end_index = end_index;
                }
            }
        }
    }
}

fn set_double_digit_palindroms(
    s: &String,
    table: &mut Vec<Vec<bool>>,
    longest_palindrome_length: &mut usize,
    result_start_index: &mut usize,
    result_end_index: &mut usize,
) {
    let symbols = s.chars().collect::<Vec<char>>();
    for index in 0..s.len() - 1 {
        let symbol = symbols[index];
        let symbol_next = symbols[index + 1];
        if symbol_next == symbol {
            table[index][index + 1] = true;
            *longest_palindrome_length = 2;
            *result_start_index = index;
            *result_end_index = index + 1;
        }
    }
}

fn set_single_digit_palindromes(length: usize, table: &mut Vec<Vec<bool>>) {
    for index in 0..length {
        table[index][index] = true;
    }
}

fn main() {
    let result = longest_palindrome(String::from("babad"));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_correct() {
        let result = longest_palindrome(String::from("babad"));
        assert_eq!(result, "aba");
    }

    #[test]
    fn when_string_is_a_palindrome_returns_string() {
        let result = longest_palindrome(String::from("babab"));
        assert_eq!(result, "babab");
    }

    #[test]
    fn when_string_2_symbols_return_correct() {
        let result = longest_palindrome(String::from("aa"));
        assert_eq!(result, "aa");
    }

    #[test]
    fn when_string_1_symbol_return_correct() {
        let result = longest_palindrome(String::from("a"));
        assert_eq!(result, "a");
    }

    #[test]
    fn when_empty_string_returns_empty() {
        let result = longest_palindrome(String::from(""));
        assert_eq!(result, "");
    }

    #[test]
    fn when_large_palindrome_returns_correct() {
        let result = longest_palindrome(String::from("kyyrjtdplseovzwjkykrjwhxquwxsfsorjiumvxjhjmgeueafubtonhlerrgsgohfosqssmizcuqryqomsipovhhodpfyudtusjhonlqabhxfahfcjqxyckycstcqwxvicwkjeuboerkmjshfgiglceycmycadpnvoeaurqatesivajoqdilynbcihnidbizwkuaoegmytopzdmvvoewvhebqzskseeubnretjgnmyjwwgcooytfojeuzcuyhsznbcaiqpwcyusyyywqmmvqzvvceylnuwcbxybhqpvjumzomnabrjgcfaabqmiotlfojnyuolostmtacbwmwlqdfkbfikusuqtupdwdrjwqmuudbcvtpieiwteqbeyfyqejglmxofdjksqmzeugwvuniaxdrunyunnqpbnfbgqemvamaxuhjbyzqmhalrprhnindrkbopwbwsjeqrmyqipnqvjqzpjalqyfvaavyhytetllzupxjwozdfpmjhjlrnitnjgapzrakcqahaqetwllaaiadalmxgvpawqpgecojxfvcgxsbrldktufdrogkogbltcezflyctklpqrjymqzyzmtlssnavzcquytcskcnjzzrytsvawkavzboncxlhqfiofuohehaygxidxsofhmhzygklliovnwqbwwiiyarxtoihvjkdrzqsnmhdtdlpckuayhtfyirnhkrhbrwkdymjrjklonyggqnxhfvtkqxoicakzsxmgczpwhpkzcntkcwhkdkxvfnjbvjjoumczjyvdgkfukfuldolqnauvoyhoheoqvpwoisniv"));
        assert_eq!(result, "qahaq");
    }
}
