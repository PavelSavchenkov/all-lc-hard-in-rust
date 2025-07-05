//! Solution for https://leetcode.com/problems/concatenated-divisibility
//! 3533. Concatenated Divisibility

impl Solution {
    pub fn concatenated_divisibility(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort();
        let n = nums.len();
        let k = k as usize;

        let mut len_merged = vec![0; 1 << n];
        for i in 0..n {
            let mut x = nums[i];
            while x > 0 {
                len_merged[1 << i] += 1;
                x /= 10;
            }
        }
        for mask in 0..(1 << n) as usize {
            if mask.count_ones() < 2 {
                continue;
            }
            let hb = 31 - (mask as u32).leading_zeros();
            len_merged[mask] = len_merged[mask ^ (1 << hb)] + len_merged[1 << hb];
        }

        let mut pw10 = vec![1 % k; len_merged[(1 << n) - 1]];
        for i in 1..pw10.len() {
            pw10[i] = (pw10[i - 1] * 10) % k;
        }

        let mut dp = vec![vec![false; k]; 1 << n];
        let mut next_id = vec![vec![0; k]; 1 << n];
        let mut next_rem = vec![vec![0; k]; 1 << n];
        for mask in 0..1 << n {
            for rem in 0..k {
                let mut ndp = false;
                if mask == 0 {
                    if rem == 0 {
                        ndp = true;
                    }
                } else {
                    for first in 0..n {
                        if ((mask >> first) & 1) == 0 {
                            continue;
                        }
                        let mask_tail = mask ^ (1 << first);
                        assert!(mask_tail < mask);
                        let first_rem = (nums[first] as usize * pw10[len_merged[mask_tail]]) % k;
                        let need_rem = (rem + k - first_rem) % k;
                        if dp[mask_tail][need_rem] {
                            next_id[mask][rem] = first;
                            next_rem[mask][rem] = need_rem;
                            ndp = true;
                            break;
                        }
                    }
                }
                dp[mask][rem] = ndp;
            }
        }

        if !dp[(1 << n) - 1][0] {
            return Vec::new();
        }

        let mut ans = Vec::new();
        let mut mask = (1 << n) - 1;
        let mut rem = 0;
        for i in 0..n {
            assert!(dp[mask][rem]);
            let id = next_id[mask][rem];
            rem = next_rem[mask][rem];
            mask ^= 1 << id;
            ans.push(nums[id]);
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
    #[case(vec![3,12,45], 5, vec![3,12,45])]
    #[case(vec![10,5], 10, vec![5,10])]
    #[case(vec![1,2,3], 5, vec![])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::concatenated_divisibility(nums, k);
        assert_eq!(actual, expected);
    }
}
