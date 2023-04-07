pub fn longest_palindrome(s: String) -> i32 {
    let mut state = [0; 64];

    for c in s.bytes() {
        state[(c & 0b111111) as usize] += 1;
    }

    let mut acc = 0;
    for count in state {
        if acc & 1 == 0  && count & 1 == 1{
            acc |= 1;
        }
        acc += count & -2;
    }

    acc
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
