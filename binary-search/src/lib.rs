use std::cmp::Ordering;

// https://leetcode.com/problems/binary-search/solutions/1641029/rust-solution/?envType=study-plan&id=level-1&languageTags=rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = (high + low) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[-1,0,3,5,9,12], 9, 4)]
    #[test_case(&[-1,0,3,5,9,12], 2, -1)]
    #[test_case(&[2, 5], 5, 1)]
    #[test_case(&[2, 5], 2, 0)]
    #[test_case(&[2, 5], 6, -1)]
    #[test_case(&[2, 5], 1, -1)]
    #[test_case(&[5], 5, 0)]
    fn examples(nums: &[i32], target: i32, index: i32) {
        assert_eq!(search(nums.to_owned(), target), index);
    }
}
