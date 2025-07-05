//! Solution for https://leetcode.com/problems/jump-game-v
//! 1340. Jump Game V

impl Solution {
    pub fn max_jumps(a: Vec<i32>, d: i32) -> i32 {
        let n = a.len();
        let d = d as usize;

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| a[i]);

        let mut dp = vec![1; n];
        for &i in &ord {
            for j in i + 1..=(i + d).min(n - 1) {
                if a[j] >= a[i] {
                    break;
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
            let lower = (i as i32 - d as i32).max(0) as usize;
            for j in (lower..i).rev() {
                if a[j] >= a[i] {
                    break;
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        let ans = *dp.iter().max().unwrap();
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
    #[case(vec![6,4,14,6,8,13,9,7,10,6,12], 2, 4)]
    #[case(vec![3,3,3,3,3], 3, 1)]
    #[case(vec![7,6,5,4,3,2,1], 1, 7)]
    fn case(#[case] arr: Vec<i32>, #[case] d: i32, #[case] expected: i32) {
        let actual = Solution::max_jumps(arr, d);
        assert_eq!(actual, expected);
    }
}
