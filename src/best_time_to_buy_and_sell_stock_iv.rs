//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv
//! 188. Best Time to Buy and Sell Stock IV

impl Solution {
    pub fn max_profit(k: i32, a: Vec<i32>) -> i32 {
        let k = k as usize;
        let n = a.len();

        // dp[i] -- max profit when i is the first untouched index
        let mut dp = vec![0; n + 1];
        for it in 0..k {
            let mut ndp = dp.clone();
            for i in 0..n {
                let mut mn = a[i];
                let mut best_deal = 0;
                for j in i + 1..n {
                    best_deal = best_deal.max(a[j] - mn);
                    mn = mn.min(a[j]);
                    ndp[j + 1] = ndp[j + 1].max(best_deal + dp[i]);
                }
            }
            dp = ndp;
        }

        dp[n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![2,4,1], 2)]
    #[case(2, vec![3,2,6,5,0,3], 7)]
    fn case(#[case] k: i32, #[case] prices: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_profit(k, prices);
        assert_eq!(actual, expected);
    }
}
