//! Solution for https://leetcode.com/problems/minimum-operations-to-form-subsequence-with-target-sum
//! 2835. Minimum Operations to Form Subsequence With Target Sum

impl Solution {
    pub fn min_operations(a: Vec<i32>, target: i32) -> i32 {
        let mut cnt = Vec::new();
        for &x in &a {
            let pw = 31 - x.leading_zeros() as usize;
            while pw >= cnt.len() {
                cnt.push(0);
            }
            cnt[pw] += 1;
        }

        let mut ans = 0;
        let pw_target = 31 - target.leading_zeros() as usize;
        for pw in 0..=pw_target {
            let bit = (target >> pw) & 1;
            if cnt[pw] == 0 && bit == 1 {
                let mut fixed = false;
                for up in pw + 1..cnt.len() {
                    if cnt[up] > 0 {
                        for i in (pw + 1..=up).rev() {
                            assert!(cnt[i] > 0);
                            cnt[i] -= 1;
                            cnt[i - 1] += 2;
                            ans += 1;
                        }
                        fixed = true;
                        break;
                    }
                }
                if !fixed {
                    return -1;
                }
            }
            if bit == 1 {
                assert!(cnt[pw] > 0);
                cnt[pw] -= 1;
            }
            let to_up = cnt[pw] / 2;
            if pw + 1 == cnt.len() {
                cnt.push(0);
            }
            cnt[pw + 1] += to_up;
            cnt[pw] %= 2;
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
    #[case(vec![1,2,8], 7, 1)]
    #[case(vec![1,32,1,2], 12, 2)]
    #[case(vec![1,32,1], 35, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::min_operations(nums, target);
        assert_eq!(actual, expected);
    }
}
