//! Solution for https://leetcode.com/problems/find-number-of-ways-to-reach-the-k-th-stair
//! 3154. Find Number of Ways to Reach the K-th Stair

const B: usize = 30;

impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let mut binom = vec![vec![0; B + 1]; B + 1];
        for n in 0..binom.len() {
            binom[n][0] = 1;
            for k in 1..=n {
                binom[n][k] = binom[n - 1][k] + binom[n - 1][k - 1];
            }
        }

        let mut ans = 0;
        for cnt in 0..=B {
            let sum = (1 << cnt) - 1;
            let ones = 1 + sum - k;
            if ones < 0 || ones > cnt as i32 + 1 {
                continue;
            }
            ans += binom[cnt + 1][ones as usize];
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
    #[case(0, 2)]
    #[case(1, 4)]
    fn case(#[case] k: i32, #[case] expected: i32) {
        let actual = Solution::ways_to_reach_stair(k);
        assert_eq!(actual, expected);
    }
}
