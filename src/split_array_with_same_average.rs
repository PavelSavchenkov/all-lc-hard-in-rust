//! Solution for https://leetcode.com/problems/split-array-with-same-average
//! 805. Split Array With Same Average

impl Solution {
    pub fn split_array_same_average(a: Vec<i32>) -> bool {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let S: usize = a.iter().sum();

        let mut dp = vec![vec![false; S + 1]; n + 1];
        dp[0][0] = true;
        for i in 0..n {
            for taken in (0..n).rev() {
                for s in (0..=S - a[i]).rev() {
                    let cur_dp = dp[taken][s];
                    if !cur_dp {
                        continue;
                    }
                    dp[taken + 1][s + a[i]] = true;
                }
            }
        }

        for taken in 1..n {
            for s in 0..=S {
                if !dp[taken][s] {
                    continue;
                }
                let other_taken = n - taken;
                let other_s = S - s;
                if other_taken * s == taken * other_s {
                    return true;
                }
            }
        }

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5,6,7,8], true)]
    #[case(vec![3,1], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::split_array_same_average(nums);
        assert_eq!(actual, expected);
    }
}
