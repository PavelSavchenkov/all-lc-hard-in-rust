//! Solution for https://leetcode.com/problems/pizza-with-3n-slices
//! 1388. Pizza With 3n Slices

impl Solution {
    pub fn max_size_slices(mut a: Vec<i32>) -> i32 {
        let n = a.len();
        a.extend_from_within(..);

        let mut dp = vec![vec![0; 2 * n]; 2 * n];
        let mut dp2 = vec![vec![0; 2 * n]; 2 * n];
        let mut dp1 = vec![vec![0; 2 * n]; 2 * n];
        for len in 1..=n {
            for l in 0..=2 * n - len {
                let r = l + len - 1;

                if (len - 1) % 3 == 0 {
                    let mut ndp = 0;
                    for i in (l..=r).step_by(3) {
                        let mut cur = 0;
                        if i > l {
                            cur += dp[l][i - 1];
                        }
                        if i < r {
                            cur += dp[i + 1][r];
                        }
                        ndp = ndp.max(cur);
                    }
                    dp1[l][r] = ndp;
                }

                if len >= 2 && (len - 2) % 3 == 0 {
                    let mut ndp = 0;
                    for i in (l..r).step_by(3) {
                        let mut cur = a[i];
                        if i > l {
                            cur += dp[l][i - 1];
                        }
                        cur += dp1[i + 1][r];
                        ndp = ndp.max(cur);
                    }
                    dp2[l][r] = ndp;
                }

                if len >= 3 && len % 3 == 0 {
                    let mut ndp = 0;
                    for i in (l..r - 1).step_by(3) {
                        // eprintln!("dp {} {} --> i = {}", l, r, i);
                        let mut cur = 0;
                        if i > l {
                            cur += dp[l][i - 1];
                        }
                        cur += dp2[i + 1][r];
                        ndp = ndp.max(cur);
                    }
                    dp[l][r] = ndp;
                }
            }
        }

        assert!(n % 3 == 0);

        let mut ans = 0;
        for l in 0..n {
            ans = ans.max(dp[l][l + n - 1]);
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
    #[case(vec![1,2,3], 3)]
    #[case(vec![1,2,3,4,5,6], 10)]
    #[case(vec![8,9,8,6,1,1], 16)]
    fn case(#[case] slices: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_size_slices(slices);
        assert_eq!(actual, expected);
    }
}
