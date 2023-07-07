pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();

    let low = 0;
    let high = nums.len() - 1;
    loop {
        let sum = (nums[low] + nums[high])/2;
        
    }

        /*
        .windows(2)
        .position(|w| w[0] + w[1] == target)
        .map(|p| vec![p as i32, (p+1) as i32])
        .unwrap()
        */
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[2,7,11,15], 9, &[0, 1])]
    #[test_case(&[3,2,4], 6, &[1, 2])]
    #[test_case(&[3,3], 6, &[0, 1])]
    fn it_works(nums: &[i32], target: i32, result: &[i32]) {
        assert_eq!(result.to_owned(), two_sum(nums.to_owned(), target));
    }
}
