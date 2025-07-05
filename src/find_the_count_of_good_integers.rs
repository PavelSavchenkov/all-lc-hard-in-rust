//! Solution for https://leetcode.com/problems/find-the-count-of-good-integers
//! 3272. Find the Count of Good Integers

use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let n = n as usize;
        let k = k as i64;

        let mut pw10 = vec![1; n + 1];
        for i in 1..pw10.len() {
            pw10[i] = pw10[i - 1] * 10 as i64;
        }

        let mut fact = vec![1; n + 1];
        for i in 2..fact.len() {
            fact[i] = fact[i - 1] * i as i64;
        }

        let mut ans = 0;
        let mut was_cnt = HashSet::new();
        let len_half = (n + 1) / 2;
        for half in 0..pw10[len_half] {
            if half % 10 == 0 {
                continue;
            }
            let mut num = half;
            let mut rem = 0;
            let mut cnt_dig = vec![0; 10];
            for i in 0..len_half {
                let dig = num % 10;
                num /= 10;
                rem = (rem + pw10[i] * dig) % k;
                cnt_dig[dig as usize] += 1;
                if i < n - 1 - i {
                    rem = (rem + pw10[n - 1 - i] * dig) % k;
                    cnt_dig[dig as usize] += 1;
                }
            }
            if rem == 0 {
                let mut h = 0;
                for d in 0..10 {
                    assert!(cnt_dig[d] <= n);
                    h = h * (n + 1) as i64 + cnt_dig[d] as i64;
                }
                if !was_cnt.insert(h) {
                    continue;
                }
                // eprintln!("cnt_dig = {:#?}", cnt_dig);
                for d in 1..10 {
                    if cnt_dig[d] > 0 {
                        cnt_dig[d] -= 1;
                        let mut cur = fact[n - 1];
                        for dd in 0..10 {
                            cur /= fact[cnt_dig[dd]];
                        }
                        ans += cur;
                        cnt_dig[d] += 1;
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
    #[case(3, 5, 27)]
    #[case(1, 4, 2)]
    #[case(5, 6, 2468)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_good_integers(n, k);
        assert_eq!(actual, expected);
    }
}
