//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii
//! 123. Best Time to Buy and Sell Stock III

fn max_diff(a: &Vec<i32>) -> Vec<i32> {
    let mut ans = 0;
    let mut all_ans = Vec::new();
    let mut mn = i32::MAX;
    for &p in a {
        if mn < p {
            ans = ans.max(p - mn);
        }
        mn = mn.min(p);
        all_ans.push(ans);
    }
    all_ans
}

impl Solution {
    pub fn max_profit(mut prices: Vec<i32>) -> i32 {
        let pref = max_diff(&prices);

        prices.reverse();
        prices = prices.iter().map(|p| -p).collect();
        let mut suff = max_diff(&prices);
        suff.reverse();

        let mut ans = 0;
        for i in 0..prices.len() {
            if i + 1 < prices.len() {
                ans = ans.max(pref[i] + suff[i + 1]);
            }
            ans = ans.max(pref[i]);
        }
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,3,5,0,0,3,1,4], 6)]
    #[case(vec![1,2,3,4,5], 4)]
    #[case(vec![7,6,4,3,1], 0)]
    fn case(#[case] prices: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_profit(prices);
        assert_eq!(actual, expected);
    }
}
