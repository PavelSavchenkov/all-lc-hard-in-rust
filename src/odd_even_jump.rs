//! Solution for https://leetcode.com/problems/odd-even-jump
//! 975. Odd Even Jump

use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

impl Solution {
    pub fn odd_even_jumps(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut dp = vec![vec![false; 2]; n];
        for it in 0..2 {
            dp[n - 1][it] = true;
        }
        let mut map = BTreeMap::<i32, usize>::new();
        map.insert(a[n - 1], n - 1);
        for i in (0..n - 1).rev() {
            dp[i][0] = {
                let range = (Unbounded, Included(a[i]));
                let mut it = map.range(range);
                let last = it.next_back();
                let mut ndp = false;
                if let Some((val, j)) = last {
                    ndp = dp[*j][1];
                }
                ndp
            };
            dp[i][1] = {
                let range = (Included(a[i]), Unbounded);
                let mut it = map.range(range);
                let last = it.next();
                let mut ndp = false;
                if let Some((val, j)) = last {
                    ndp = dp[*j][0];
                }
                ndp
            };
            map.insert(a[i], i);
        }
        let mut ans = 0;
        for i in 0..n {
            if dp[i][1] {
                ans += 1;
            }
        }
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![10,13,12,14,15], 2)]
    #[case(vec![2,3,1,1,4], 3)]
    #[case(vec![5,1,3,4,2], 3)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::odd_even_jumps(arr);
        assert_eq!(actual, expected);
    }
}
