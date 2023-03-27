fn main() {
    println!("Hello, world!");
}


pub fn sum(nums: &[i32]) -> i32 {
    nums.into_iter().sum()
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let before = nums.clone();
    let after = nums.clone();
    let length = nums.len();

    for i in 0..length {
        if sum(&before[0..i]) == sum(&after[(i+1)..length]) {
            return i.try_into().unwrap();
        }
    }
    -1
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
