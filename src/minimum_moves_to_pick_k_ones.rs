//! Solution for https://leetcode.com/problems/minimum-moves-to-pick-k-ones
//! 3086. Minimum Moves to Pick K Ones

impl Solution {
    pub fn minimum_moves(a: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let n = a.len();
        let k = k as usize;
        let max_changes = (max_changes as usize).min(k);

        let mut ps = Vec::new();
        for i in 0..n {
            if a[i] == 1 {
                ps.push(i);
            }
        }

        let mut pref = vec![0; ps.len() + 1];
        for i in 0..ps.len() {
            pref[i + 1] = pref[i] + ps[i] as i64;
        }

        let mut ans = i64::MAX;
        let L = (max_changes as i32 - 3).max(0) as usize;
        let R = max_changes;
        for changes in L..=R {
            let len = k - changes;
            if len > ps.len() {
                continue;
            }
            if len == 0 {
                ans = ans.min(changes as i64 * 2);
            } else {
                for i in 0..=ps.len() - len {
                    let l = i;
                    let r = i + len - 1;
                    let m = (l + r) / 2;

                    let mut cur = 0;
                    cur += ps[m] as i64 * (m - l + 1) as i64 - (pref[m + 1] - pref[l]);
                    cur += (pref[r + 1] - pref[m + 1]) - ps[m] as i64 * (r - (m + 1) + 1) as i64;
                    cur += changes as i64 * 2;
                    ans = ans.min(cur);
                }
            }
        }
        assert!(ans < i64::MAX);
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
    #[case(vec![1,1,0,0,0,1,1,0,0,1], 3, 1, 3)]
    #[case(vec![0,0,0,0], 2, 3, 4)]
    #[case(vec![1,1,1,1,1], 5, 1, 0)]
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] max_changes: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::minimum_moves(nums, k, max_changes);
        assert_eq!(actual, expected);
    }
}
