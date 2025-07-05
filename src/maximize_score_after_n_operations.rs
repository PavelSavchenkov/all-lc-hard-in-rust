//! Solution for https://leetcode.com/problems/maximize-score-after-n-operations
//! 1799. Maximize Score After N Operations

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 && b != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn max_score(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut dp = vec![0; 1 << n];
        for mask in 0..((1 << n) as usize) {
            let mut coef = mask.count_ones() as i32;
            if coef % 2 != 0 {
                continue;
            }
            coef = coef / 2 + 1;
            let cur_dp = dp[mask];
            for i in 0..n {
                if ((mask >> i) & 1) == 1 {
                    continue;
                }
                for j in 0..i {
                    if ((mask >> j) & 1) == 1 {
                        continue;
                    }
                    let cur = coef * gcd(a[i], a[j]);
                    remax(&mut dp[mask | (1 << i) | (1 << j)], cur_dp + cur);
                }
            }
        }
        dp[(1 << n) - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2], 1)]
    #[case(vec![3,4,6,8], 11)]
    #[case(vec![1,2,3,4,5,6], 14)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_score(nums);
        assert_eq!(actual, expected);
    }
}
