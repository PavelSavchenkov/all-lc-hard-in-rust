//! Solution for https://leetcode.com/problems/minimum-increments-for-target-multiples-in-an-array
//! 3444. Minimum Increments for Target Multiples in an Array

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a > 0 && b > 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

impl Solution {
    pub fn minimum_increments(a: Vec<i32>, target: Vec<i32>) -> i32 {
        let n = a.len();

        let m = target.len();
        let mut lcm_sub = vec![0; 1 << m];
        for mask in 0..1 << m {
            let mut l = 1;
            for i in 0..m {
                if ((mask >> i) & 1) == 1 {
                    l = lcm(l, target[i] as i64);
                }
            }
            lcm_sub[mask] = l;
        }

        let mut dp = vec![vec![i64::MAX; 1 << m]; n + 1];
        dp[0][0] = 0;
        for i in 0..n {
            let have = a[i] as i64;
            for covered in 0..1 << m {
                let cur_dp = dp[i][covered];
                if cur_dp == i64::MAX {
                    continue;
                }
                let uncovered = !covered & ((1 << m) - 1);
                let mut mask = uncovered;
                loop {
                    let need = lcm_sub[mask];
                    let ndp = cur_dp + (need - have % need) % need;
                    dp[i + 1][covered | mask] = dp[i + 1][covered | mask].min(ndp);
                    if mask == 0 {
                        break;
                    }
                    mask = (mask - 1) & uncovered;
                }
            }
        }

        dp[n][(1 << m) - 1] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![4], 1)]
    #[case(vec![8,4], vec![10,5], 2)]
    #[case(vec![7,9,10], vec![7], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] target: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_increments(nums, target);
        assert_eq!(actual, expected);
    }
}
