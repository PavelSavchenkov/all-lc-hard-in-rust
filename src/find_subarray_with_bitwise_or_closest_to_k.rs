//! Solution for https://leetcode.com/problems/find-subarray-with-bitwise-or-closest-to-k
//! 3171. Find Subarray With Bitwise OR Closest to K

const B: usize = 32;

impl Solution {
    pub fn minimum_difference(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();

        let mut ans = i32::MAX;
        let mut last = vec![n; B];
        for i in (0..n).rev() {
            for b in 0..B {
                if ((a[i] >> b) & 1) == 1 {
                    last[b] = i;
                }
            }
            let mut bits: Vec<_> = (0..B).collect();
            bits.sort_by_key(|&b| last[b]);

            let mut or = 0;
            for j in 0..B {
                if j > 0 && last[bits[j]] != last[bits[j - 1]] {
                    ans = ans.min((k - or).abs());
                }
                or |= 1 << bits[j];
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
    #[case(vec![1,2,4,5], 3, 0)]
    #[case(vec![1,3,1,3], 2, 1)]
    #[case(vec![1], 10, 9)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_difference(nums, k);
        assert_eq!(actual, expected);
    }
}
