//! Solution for https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-ii
//! 3266. Final Array State After K Multiplication Operations II

use std::collections::BTreeSet;

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut a: i64, mut p: u64) -> i64 {
    a = a % MOD;
    let mut res = 1;
    while p > 0 {
        if p % 2 == 1 {
            res = (res * a) % MOD;
        }
        a = (a * a) % MOD;
        p /= 2;
    }
    res
}

impl Solution {
    pub fn get_final_state(a: Vec<i32>, mut k: i32, m: i32) -> Vec<i32> {
        let mut a: Vec<_> = a.iter().map(|&x| x as i64).collect();
        let n = a.len();
        let m = m as i64;

        if m == 1 {
            k = 0;
        }

        let mut set = BTreeSet::new();
        for i in 0..n {
            set.insert((a[i], i));
        }
        while k > 0 {
            let max = set.last().unwrap().0;
            let min = set.first().unwrap().0;
            if max / min < m {
                break;
            }
            let i = set.pop_first().unwrap().1;
            k -= 1;
            a[i] *= m;
            set.insert((a[i], i));
        }

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| a[i]); // stable

        let full_passes = k / (n as i32);
        k -= full_passes * n as i32;
        let full_mult = pow_mod(m, full_passes as u64);
        let mut ans = vec![0; n];
        for &i in &ord {
            let mut x = a[i] % MOD;
            x = (x * full_mult) % MOD;
            if k > 0 {
                x = (x * m) % MOD;
                k -= 1;
            }
            ans[i] = x as i32;
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
    #[case(vec![2,1,3,5,6], 5, 2, vec![8,4,6,5,6])]
    #[case(vec![100000,2000], 2, 1000000, vec![999999307,999999993])]
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] multiplier: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::get_final_state(nums, k, multiplier);
        assert_eq!(actual, expected);
    }
}
