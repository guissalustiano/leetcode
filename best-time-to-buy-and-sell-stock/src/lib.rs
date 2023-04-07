struct Bigger(i32);
impl Bigger {
    pub fn new() -> Self {
        Bigger(i32::MIN)
    }

    pub fn push(&mut self, nv: i32) {
        if nv > self.0 {
           self.0 = nv;
        }
    }
}

struct Lower(i32);
impl Lower {
    pub fn new() -> Self {
        Lower(i32::MAX)
    }

    pub fn push(&mut self, nv: i32) {
        if nv < self.0 {
            self.0 = nv;
        }
    }
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut lower = Lower::new();
    let mut profit = Bigger::new();

    prices.into_iter().for_each(|p| {
       lower.push(p);
       profit.push(p - lower.0);
    });

    profit.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn example_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn example_3() {
        let prices = vec![3,8,1,4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn example_6() {
        let prices = vec![2,1,2,1,0,1,2];
        assert_eq!(max_profit(prices), 2);
    }

    #[test]
    fn example_4() {
        let prices: Vec<_> = (0..=10000000).collect();
        assert_eq!(max_profit(prices), 10000000);
    }
}
