//! Solution for https://leetcode.com/problems/count-subarrays-with-score-less-than-k
//! 2302. Count Subarrays With Score Less Than K

impl Solution {
    pub fn count_subarrays(a: Vec<i32>, k: i64) -> i64 {
        let a: Vec<_> = a.iter().map(|&x| x as i64).collect();
        let n = a.len();

        let mut ans = 0;
        let mut j = 0;
        let mut sum = 0;
        for i in 0..n {
            while j < n && ((sum + a[j]) * (j - i + 1) as i64) < k {
                sum += a[j];
                j += 1;
            }
            ans += (j - i) as i64;
            if j > i {
                sum -= a[i];
            } else {
                j = i + 1;
                sum = 0;
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
    #[case(vec![2,1,4,3,5], 10, 6)]
    #[case(vec![1,1,1], 5, 5)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i64, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
