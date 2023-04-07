pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).map(|x| x as i32).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[-1,0,3,5,9,12], 9, 4)]
    #[test_case(&[-1,0,3,5,9,12], 2, -1)]
    fn examples(nums: &[i32], target: i32, index: i32) {
        assert_eq!(search(nums.to_owned(), target), index);
    }
}
