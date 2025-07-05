//! Solution for https://leetcode.com/problems/maximum-product-of-subsequences-with-an-alternating-sum-equal-to-k
//! 3509. Maximum Product of Subsequences With an Alternating Sum Equal to K

impl Solution {
    pub fn max_product(mut a: Vec<i32>, k: i32, limit: i32) -> i32 {
        let mut n = a.len();

        let limit = limit as usize;

        let mut ans = -1;
        // check prod == 0
        {
            let max_sum: usize = 150 * 12 / 2;

            if k.abs() as usize > max_sum {
                return -1;
            }

            let pos_dp = |sum: usize, was_zero: usize, cnt_mod_2: usize| -> usize {
                sum * 2 * 2 + was_zero * 2 + cnt_mod_2
            };

            // dp[sum, |sum| <= max_sum][was_zero][cnt_mod_2] -- true/false
            let dp_size = (2 * max_sum + 1) * 2 * 2;
            let mut dp = vec![false; dp_size];
            let mut ndp = vec![false; dp_size];
            dp[pos_dp(max_sum, 0, 0)] = true;
            for i in 0..n {
                ndp.copy_from_slice(&dp[..]);
                for sum in 0..=2 * max_sum {
                    for was_zero in 0..2 {
                        for cnt_mod_2 in 0..2 {
                            let cur_dp = dp[pos_dp(sum, was_zero, cnt_mod_2)];
                            if !cur_dp {
                                continue;
                            }
                            // take next number
                            let new_sum = sum as i32 + (if cnt_mod_2 == 0 { a[i] } else { -a[i] });
                            assert!(new_sum >= 0);
                            let new_sum = new_sum as usize;
                            assert!(new_sum <= 2 * max_sum);
                            let mut new_was_zero = was_zero;
                            if a[i] == 0 {
                                new_was_zero = 1;
                            }
                            ndp[pos_dp(new_sum, new_was_zero, cnt_mod_2 ^ 1)] = true;
                        }
                    }
                }
                dp.copy_from_slice(&ndp[..]);
            }
            for par in 0..2 {
                if dp[pos_dp((max_sum as i32 + k) as usize, 1, par)] {
                    ans = 0;
                }
            }
            a = a.iter().filter(|&&x| x != 0).copied().collect();
            n = a.len();
        }

        let mut max_sum = 0;
        for val in 2..=12 {
            let mut cnt = 0;
            let mut cur = 1;
            while cur <= limit {
                cur *= val;
                cnt += 1;
            }
            assert!(cnt > 0);
            cnt -= 1;
            let sum = (cnt + 1) + cnt * val;
            max_sum = max_sum.max(sum);
        }
        let max_sum = max_sum as usize;

        if k.abs() as usize <= max_sum {
            let pos_dp = |prod: usize, sum: usize, cnt_mod_2: usize, is_empty: usize| -> usize {
                prod * (2 * max_sum + 1) * 2 * 2 + sum * 2 * 2 + cnt_mod_2 * 2 + is_empty
            };

            // dp[prod <= limit][sum, |sum| <= max_sum][cnt_mod_2][is_empty] -- true/false
            let dp_size = (limit + 1) * (2 * max_sum + 1) * 2 * 2;
            let mut dp = vec![false; dp_size];
            let mut ndp = vec![false; dp_size];
            dp[pos_dp(1, max_sum, 0, 1)] = true;
            for i in 0..n {
                ndp.copy_from_slice(&dp[..]);
                for prod in 0..=limit {
                    for sum in 0..=2 * max_sum {
                        for cnt_mod_2 in 0..2 {
                            for is_empty in 0..2 {
                                let cur_dp = dp[pos_dp(prod, sum, cnt_mod_2, is_empty)];
                                if !cur_dp {
                                    continue;
                                }
                                // take next number
                                let new_prod = prod * a[i] as usize;
                                if new_prod > limit {
                                    continue;
                                }
                                let new_sum =
                                    sum as i32 + (if cnt_mod_2 == 0 { a[i] } else { -a[i] });
                                assert!(new_sum >= 0);
                                let new_sum = new_sum as usize;
                                assert!(new_sum <= 2 * max_sum);
                                ndp[pos_dp(new_prod, new_sum as usize, cnt_mod_2 ^ 1, 0)] = true;
                            }
                        }
                    }
                }
                dp.copy_from_slice(&ndp[..]);
            }

            for prod in (0..=limit).rev() {
                for par in 0..2 {
                    if dp[pos_dp(prod, (max_sum as i32 + k) as usize, par, 0)] {
                        ans = ans.max(prod as i32);
                    }
                }
            }
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
    #[case(vec![1,2,3], 2, 10, 6)]
    #[case(vec![0,2,3], -5, 12, -1)]
    #[case(vec![2,2,3,3], 0, 9, 9)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::max_product(nums, k, limit);
        assert_eq!(actual, expected);
    }
}
