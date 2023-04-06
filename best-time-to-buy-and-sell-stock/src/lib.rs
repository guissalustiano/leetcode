
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut lower: Option<i32> = None;
    let mut bigger: Option<i32> = None;

    prices.into_iter().for_each(|v|{
        if let Some(l) = lower {
            if v < l {
                lower = Some(v);
            }

            if v > bigger.unwrap_or(l) {
                bigger = Some(v);
            }
        } else {
            lower = Some(v);
        }
    });

    match (lower, bigger) {
        (Some(l), Some(b)) => b - l,
        _ => 0
    }
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

        //  0,  5, -2,  1
        //  _,  0, -7, -4
        //  _,  _,  0,  3
        //  _,  _,  _,  0

        assert_eq!(max_profit(prices), 5);
    }
}
