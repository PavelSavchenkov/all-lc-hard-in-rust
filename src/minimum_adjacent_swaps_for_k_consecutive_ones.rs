//! Solution for https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones
//! 1703. Minimum Adjacent Swaps for K Consecutive Ones

fn sum_arith(l: i64, r: i64) -> i64 {
    if l > r {
        return 0;
    }
    (l + r) * (r - l + 1) / 2
}

impl Solution {
    pub fn min_moves(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = a.len();

        let mut p = Vec::new();
        for i in 0..n {
            if a[i] == 1 {
                p.push(i);
            }
        }

        let mut pref = vec![0 as i64; p.len() + 1];
        for i in 0..p.len() {
            pref[i + 1] = pref[i] + p[i] as i64;
        }

        let mut ans = i64::MAX;
        for l in 0..=p.len() - k {
            let r = l + k - 1;
            assert!(r < p.len());
            let m = (l + r) / 2;

            let mut cur = 0;
            cur -= pref[m + 1] - pref[l];
            cur += p[m] as i64 * (m - l + 1) as i64;
            cur -= sum_arith(1, (m - l) as i64);

            cur += pref[r + 1] - pref[m];
            cur -= p[m] as i64 * (r - m + 1) as i64;
            cur -= sum_arith(1, (r - m) as i64);

            ans = ans.min(cur);
        }
        assert!(ans < i32::MAX as i64);
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
    #[case(vec![1,0,0,1,0,1], 2, 1)]
    #[case(vec![1,0,0,0,0,0,1,1], 3, 5)]
    #[case(vec![1,1,0,1], 2, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_moves(nums, k);
        assert_eq!(actual, expected);
    }
}
