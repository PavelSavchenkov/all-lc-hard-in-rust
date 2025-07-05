//! Solution for https://leetcode.com/problems/super-egg-drop
//! 887. Super Egg Drop

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;

        // min L such that binom(L, 0) + binom(L, 1) + ... + binom(L, k) >= n
        // L <= n
        let mut binom = vec![vec![0; k + 1]; n + 1];
        for L in 0..=n {
            let mut sum = 0;
            binom[L][0] = 1;
            for i in 1..=k.min(L) {
                binom[L][i] = binom[L - 1][i] + binom[L - 1][i - 1];
                sum += binom[L][i];
            }
            if sum >= n {
                return L as i32;
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 2, 2)]
    #[case(2, 6, 3)]
    #[case(3, 14, 4)]
    fn case(#[case] k: i32, #[case] n: i32, #[case] expected: i32) {
        let actual = Solution::super_egg_drop(k, n);
        assert_eq!(actual, expected);
    }
}
