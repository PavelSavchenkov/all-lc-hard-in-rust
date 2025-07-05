//! Solution for https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden
//! 1326. Minimum Number of Taps to Open to Water a Garden

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let n = n as usize;

        let mut dp = vec![usize::MAX; n + 1];
        dp[0] = 0;
        for i in 0..=n {
            let len = ranges[i];
            let l = (i as i32 - len).max(0) as usize;
            let r = (i + len as usize).min(n);
            if l == r {
                continue;
            }
            let min = *dp[l..r].iter().min().unwrap();
            if min < usize::MAX {
                dp[r] = dp[r].min(min + 1);
            }
        }
        dp[n] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![3,4,1,1,0,0], 1)]
    #[case(3, vec![0,0,0,0], -1)]
    fn case(#[case] n: i32, #[case] ranges: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_taps(n, ranges);
        assert_eq!(actual, expected);
    }
}
