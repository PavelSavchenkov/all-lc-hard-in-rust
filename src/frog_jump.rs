//! Solution for https://leetcode.com/problems/frog-jump
//! 403. Frog Jump

use std::collections::HashMap;

impl Solution {
    pub fn can_cross(a: Vec<i32>) -> bool {
        let n = a.len();

        let mut pos_of = HashMap::new();
        for i in 0..n {
            pos_of.insert(a[i], i);
        }

        let mut dp = vec![vec![false; n + 1]; n];
        dp[0][1] = true;
        for i in 0..n {
            for next_k in 1..=i + 1 {
                if !dp[i][next_k] {
                    continue;
                }
                let next_x = a[i] + next_k as i32;
                let next_i = pos_of.get(&next_x);
                if next_i.is_none() {
                    continue;
                }
                let next_i = *next_i.unwrap();
                for next_next_k in (next_k - 1).max(1)..=next_k + 1 {
                    dp[next_i][next_next_k] = true;
                }
            }
        }
        dp[n - 1].iter().any(|&d| d)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,3,5,6,8,12,17], true)]
    #[case(vec![0,1,2,3,4,8,9,11], false)]
    fn case(#[case] stones: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_cross(stones);
        assert_eq!(actual, expected);
    }
}
