pub fn longest_palindrome(s: String) -> i32 {
    s.bytes().fold([0; 255], |mut state, c| {
        state[c as usize] += 1;
        state
    }).into_iter().fold(0, |mut acc, count| {
        // the palindrome could have one letter leftover
        // here we check if already count this
        if count % 2 == 1 {
            if acc % 2 == 0  {
                acc += 1;
            }
            acc += count - 1;
        } else {
            acc += count;
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abccccdd", 7)]
    #[test_case("a", 1)]
    fn examples(s: &str, expected: i32) {
        assert_eq!(longest_palindrome(s.to_string()), expected);
    }
}
