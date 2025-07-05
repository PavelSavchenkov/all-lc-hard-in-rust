//! Solution for https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares
//! 2897. Apply Operations on Array to Maximize Sum of Squares

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_sum(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut cnt = vec![0; 32];
        for &x in &a {
            for b in 0..32 {
                if ((x >> b) & 1) == 1 {
                    cnt[b] += 1;
                }
            }
        }

        let mut ans = 0;
        for it in 0..k {
            let mut num = 0;
            for b in 0..32 {
                if cnt[b] > 0 {
                    num ^= 1 << b;
                    cnt[b] -= 1;
                }
            }
            ans = (ans + num as i64 * num as i64) % MOD;
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
    #[case(vec![2,6,5,8], 2, 261)]
    #[case(vec![4,5,4,7], 3, 90)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
