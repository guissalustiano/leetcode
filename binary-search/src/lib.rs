pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut botton = 0;
    let mut top = nums.len();
    loop {
        let guess = (botton + top)/2;
        match nums[guess].cmp(&target) {
            std::cmp::Ordering::Less => {
                if botton == guess { return -1 } 
                botton = guess
            },
            std::cmp::Ordering::Greater => {
                if top == guess { return -1 } 
                top = guess
            },
            std::cmp::Ordering::Equal => return guess as i32,
        }
    }
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
