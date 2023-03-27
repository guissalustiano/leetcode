pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().scan(0, |state, n| {
            *state += n;
            Some(*state)
        }).collect()
    }

pub fn running_sum_v2(nums: Vec<i32>) -> Vec<i32> {
        let mut acc = 0;

        nums.iter().map(|n| {
            acc += n;
            acc
        }).collect()
    }

pub fn running_sum_v1(nums: Vec<i32>) -> Vec<i32> {
        let mut acc = 0;
        let mut v = Vec::with_capacity(nums.len());

        for num in nums {
            acc += num;
            v.push(acc);
        }

        v
    }

fn main() {
    println!("Hello, world!");
}

#[test]
fn exemple1() {
    assert_eq!(running_sum(vec![1,2,3,4]), vec![1,3,6,10]);
}

#[test]
fn exemple2() {
    assert_eq!(running_sum(vec![1,1,1,1,1]), vec![1,2,3,4,5]);
}

#[test]
fn exemple3() {
    assert_eq!(running_sum(vec![3,1,2,10,1]), vec![3,4,6,16,17]);
}
