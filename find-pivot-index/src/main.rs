fn main() {
    println!("Hello, world!");
}

trait ScanSum {
    fn scan_sum<'a>(self) -> std::iter::Scan<Self, i32, fn(&mut i32, &i32) -> Option<i32>>
        where
            Self : Sized
        ;
}

impl<'a, T> ScanSum for T 
where
    T: Iterator<Item = &'a i32>,
    <T as Iterator>::Item: Clone,
{
    #[inline]
    fn scan_sum(self) -> std::iter::Scan<Self, i32, fn(&mut i32, &i32) -> Option<i32>>
    {
        self.scan(0, |state, n| {
            *state += n;
            Some(*state)
        })
    }
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let acc_left = nums.iter().scan_sum();
    let acc_right = nums.iter().rev().scan_sum().collect::<Vec<_>>();

    acc_left
        .zip(acc_right.into_iter().rev())
        .position(|(sum_left, sum_right)|  sum_left == sum_right)
        .map(|x| x as i32)
        .unwrap_or(-1)
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
