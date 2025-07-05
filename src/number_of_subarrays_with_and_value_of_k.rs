//! Solution for https://leetcode.com/problems/number-of-subarrays-with-and-value-of-k
//! 3209. Number of Subarrays With AND Value of K

const B: usize = 32;

impl Solution {
    pub fn count_subarrays(a: Vec<i32>, k: i32) -> i64 {
        let n = a.len();

        let mut ans = 0;
        let mut last = vec![n; B];
        for i in (0..n).rev() {
            for b in 0..B {
                if ((a[i] >> b) & 1) == 0 {
                    last[b] = i;
                }
            }

            let mut ids = last.clone();
            ids.push(n);
            ids.push(i);
            ids.sort();
            ids.dedup();

            let mut and = a[i];
            for j in 0..ids.len() - 1 {
                and &= a[ids[j]];
                if and == k {
                    ans += (ids[j + 1] - ids[j]) as i64;
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
    #[case(vec![1,1,1], 1, 6)]
    #[case(vec![1,1,2], 1, 3)]
    #[case(vec![1,2,3], 2, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
