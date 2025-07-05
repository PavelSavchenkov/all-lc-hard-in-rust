//! Solution for https://leetcode.com/problems/count-non-decreasing-subarrays-after-k-operations
//! 3420. Count Non-Decreasing Subarrays After K Operations

impl Solution {
    pub fn count_non_decreasing_subarrays(a: Vec<i32>, k: i32) -> i64 {
        let n = a.len();
        let k = k as i64;

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }

        let mut ans = 0;
        let mut ops = 0;
        let mut st = Vec::<usize>::new();
        let mut R = n;
        let mut L = n;
        for i in (0..n).rev() {
            while !st.is_empty() && a[*st.last().unwrap()] <= a[i] {
                let j = st.pop().unwrap();
                let p = if st.is_empty() {
                    n
                } else {
                    *st.last().unwrap()
                };
                assert!(j < p);
                let l = j.max(L);
                let r = p.min(R);
                if l < r {
                    ops -= a[j] as i64 * (r - l) as i64 - (pref[r] - pref[l]);
                }
            }
            let p = if st.is_empty() {
                n
            } else {
                *st.last().unwrap()
            };
            assert!(i < p);
            let l = (i + 1).max(L);
            let r = p.min(R);
            if l < r {
                ops += a[i] as i64 * (r - l) as i64 - (pref[r] - pref[l]);
            }
            st.push(i);
            assert!(L == i + 1);
            L -= 1;

            while ops > k {
                let z = R - 1;
                let pos = st.partition_point(|&j| j > z);
                let j = st[pos];
                assert!(j <= z);
                assert!(a[j] >= a[z]);
                ops -= a[j] as i64 - a[z] as i64;
                R -= 1;
            }
            // eprintln!("i={}, L={}, R={}, ops={}", i, L, R, ops);
            ans += (R - L) as i64;
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
    #[case(vec![6,3,1,2,4,4], 7, 17)]
    #[case(vec![6,3,1,3,6], 4, 12)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_non_decreasing_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
