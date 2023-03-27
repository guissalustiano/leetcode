fn main() {
    println!("Hello, world!");
}

// from running-sum-of-1d-array exercise 
#[inline]
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().scan(0, |state, n| {
            *state += n;
            Some(*state)
        }).collect()
    }

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let acc_left = nums.iter().scan(0, |state, n| {
            *state += n;
            Some(*state)
        }).collect::<Vec<_>>();

    let acc_right = nums.iter().rev().scan(0, |state, n| {
            *state += n;
            Some(*state)
        }).collect::<Vec<_>>();

    acc_left.into_iter()
        .zip(acc_right.into_iter().rev())
        .enumerate()
        .find_map(|(index, (sum_left, sum_right))| {
            if sum_left == sum_right { Some(index as i32) } else { None }
        }).unwrap_or(-1)
}

#[test]
fn example1() {
    assert_eq!(pivot_index(vec![1,7,3,6,5,6]), 3);
}

#[test]
fn example2() {
    assert_eq!(pivot_index(vec![1,2,3]), -1);
}

#[test]
fn example3() {
    assert_eq!(pivot_index(vec![2,1,-1]), 0);
}
