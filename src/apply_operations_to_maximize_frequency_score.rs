//! Solution for https://leetcode.com/problems/apply-operations-to-maximize-frequency-score
//! 2968. Apply Operations to Maximize Frequency Score

impl Solution {
    pub fn max_frequency_score(mut a: Vec<i32>, k: i64) -> i32 {
        let n = a.len();
        a.sort();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }

        let can = |len: usize| -> bool {
            assert!(len <= n);
            for l in 0..=n - len {
                let r = l + len - 1;
                assert!(r < n);
                let mid = (l + r) / 2;
                let mut need = (mid - l + 1) as i64 * a[mid] as i64 - (pref[mid + 1] - pref[l]);
                need += (pref[r + 1] - pref[mid + 1]) - (r - mid) as i64 * a[mid] as i64;
                if need <= k {
                    return true;
                }
            }
            false
        };

        let mut L = 0;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                L = M;
            } else {
                R = M;
            }
        }
        L as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,6,4], 3, 3)]
    #[case(vec![1,4,4,2,4], 0, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i64, #[case] expected: i32) {
        let actual = Solution::max_frequency_score(nums, k);
        assert_eq!(actual, expected);
    }
}
