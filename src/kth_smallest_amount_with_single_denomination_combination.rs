//! Solution for https://leetcode.com/problems/kth-smallest-amount-with-single-denomination-combination
//! 3116. Kth Smallest Amount With Single Denomination Combination

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a > 0 && b > 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let n = coins.len();

        let mut lcm = vec![1; 1 << n];
        for mask in 1..((1 << n) as usize) {
            let hb = mask.trailing_zeros() as usize;
            let prev_mask = mask ^ (1 << hb);
            assert!(prev_mask < mask);
            let prev = lcm[prev_mask];
            let c = coins[hb] as i64;
            lcm[mask] = (c / gcd(c as i64, prev)) * prev;
        }

        let cnt = |x: i64| -> i64 {
            let mut ans = 0;
            for mask in 1..((1 << n) as usize) {
                let coef = x / lcm[mask];
                if mask.count_ones() % 2 == 1 {
                    ans += coef;
                } else {
                    ans -= coef;
                }
            }
            ans
        };

        let mut L = 0;
        let mut R = k as i64 * 25 + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if cnt(M) < k as i64 {
                L = M;
            } else {
                R = M;
            }
        }
        R
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,6,9], 3, 9)]
    #[case(vec![5,2], 7, 12)]
    fn case(#[case] coins: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::find_kth_smallest(coins, k);
        assert_eq!(actual, expected);
    }
}
