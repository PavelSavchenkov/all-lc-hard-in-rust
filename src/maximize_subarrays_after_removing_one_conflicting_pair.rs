//! Solution for https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair
//! 3480. Maximize Subarrays After Removing One Conflicting Pair

fn remax(a: &mut i64, b: i64) {
    if *a < b {
        *a = b;
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;

        let mut mx1 = vec![-1; n];
        let mut mx2 = vec![-1; n];
        for p in &conflicting_pairs {
            let mut a = p[0] as usize;
            let mut b = p[1] as usize;
            a -= 1;
            b -= 1;
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            let a = a as i32;
            if a > mx1[b] {
                mx2[b] = mx1[b];
                mx1[b] = a;
            } else if a > mx2[b] {
                mx2[b] = a;
            }
        }

        let mut dp = vec![vec![HashMap::<i32, i64>::new(); 2]; n + 1];
        dp[0][0].insert(-1, 0);
        for i in 0..n {
            for del in 0..2 {
                assert!(dp[i][del].len() <= 2);
                let cur_dp: Vec<(i32, i64)> = dp[i][del].iter().map(|(k, v)| (*k, *v)).collect();
                for &(pref_mx, max_cnt) in &cur_dp {
                    // do not delete
                    let new_pref_mx = pref_mx.max(mx1[i]);
                    let ndp = max_cnt + (i as i32 - new_pref_mx) as i64;
                    remax(&mut dp[i + 1][del].entry(new_pref_mx).or_insert(0), ndp);
                    // delete
                    if del == 0 {
                        let new_pref_mx = pref_mx.max(mx2[i]);
                        let ndp = max_cnt + (i as i32 - new_pref_mx) as i64;
                        remax(&mut dp[i + 1][1].entry(new_pref_mx).or_insert(0), ndp);
                    }
                }
            }
        }
        let mut ans = 0;
        for (_, &max_cnt) in &dp[n][1] {
            ans = ans.max(max_cnt);
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
    #[case(4, vec![vec![2,3],vec![1,4]], 9)]
    #[case(5, vec![vec![1,2],vec![2,5],vec![3,5]], 12)]
    fn case(#[case] n: i32, #[case] conflicting_pairs: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(actual, expected);
    }
}
