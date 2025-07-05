//! Solution for https://leetcode.com/problems/count-increasing-quadruplets
//! 2552. Count Increasing Quadruplets

impl Solution {
    pub fn count_quadruplets(a: Vec<i32>) -> i64 {
        let n = a.len();

        let mut dp_lower = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..i {
                if a[j] < a[i] {
                    dp_lower[i][j] += 1;
                }
                if j > 0 {
                    dp_lower[i][j] += dp_lower[i][j - 1];
                }
            }
        }
        let mut dp_upper = vec![vec![0; n]; n];
        for i in 0..n {
            for j in (i + 1..n).rev() {
                if a[j] > a[i] {
                    dp_upper[i][j] += 1;
                }
                if j + 1 < n {
                    dp_upper[i][j] += dp_upper[i][j + 1];
                }
            }
        }

        let mut ans: i64 = 0;
        for i in 1..n {
            for j in i + 1..n - 1 {
                if !(a[i] > a[j]) {
                    continue;
                }
                let left = dp_lower[j][i - 1] as i64;
                let right = dp_upper[i][j + 1] as i64;
                ans += left * right;
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
    #[case(vec![1,3,2,4,5], 2)]
    #[case(vec![1,2,3,4], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::count_quadruplets(nums);
        assert_eq!(actual, expected);
    }
}
