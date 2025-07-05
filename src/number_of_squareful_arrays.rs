//! Solution for https://leetcode.com/problems/number-of-squareful-arrays
//! 996. Number of Squareful Arrays

fn is_square(x: u32) -> bool {
    let mut s = (x as f64).sqrt() as u32;
    while s * s < x {
        s += 1;
    }
    while s * s > x {
        s -= 1;
    }
    s * s == x
}

impl Solution {
    pub fn num_squareful_perms(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut is_squareful = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..i {
                let ok = is_square(a[i] as u32 + a[j] as u32);
                is_squareful[i][j] = ok;
                is_squareful[j][i] = ok;
            }
        }

        let mut mask_after = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if a[j] == a[i] {
                    mask_after[i] ^= 1 << j;
                }
            }
        }

        let mut dp = vec![vec![0; n]; 1 << n];
        for i in 0..n {
            if mask_after[i] == 0 {
                dp[1 << i][i] = 1;
            }
        }
        for mask in 1..1 << n {
            for last in 0..n {
                if ((mask >> last) & 1) == 0 {
                    continue;
                }
                let cur_dp = dp[mask][last];
                if cur_dp == 0 {
                    continue;
                }
                for next in 0..n {
                    if ((mask >> next) & 1) == 1 {
                        continue;
                    }
                    if !is_squareful[last][next] {
                        continue;
                    }
                    if (mask & mask_after[next]) != mask_after[next] {
                        continue;
                    }
                    dp[mask ^ (1 << next)][next] += cur_dp;
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            ans += dp[(1 << n) - 1][i];
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
    #[case(vec![1,17,8], 2)]
    #[case(vec![2,2,2], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_squareful_perms(nums);
        assert_eq!(actual, expected);
    }
}
