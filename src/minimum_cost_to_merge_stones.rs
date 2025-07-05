//! Solution for https://leetcode.com/problems/minimum-cost-to-merge-stones
//! 1000. Minimum Cost to Merge Stones

impl Solution {
    pub fn merge_stones(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;

        if (n - k) % (k - 1) != 0 {
            return -1;
        }

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i];
        }

        let mut dp = vec![vec![vec![i32::MAX; k + 1]; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                for cnt in 2..=k {
                    for rr in l..r {
                        let left = dp[l][rr][cnt - 1];
                        let right = dp[rr + 1][r][1];
                        if left == i32::MAX || right == i32::MAX {
                            continue;
                        }
                        let ndp = left + right;
                        dp[l][r][cnt] = dp[l][r][cnt].min(ndp);
                    }
                }
                if len == 1 {
                    dp[l][r][1] = 0;
                } else {
                    let sum = pref[r + 1] - pref[l];
                    let split_k = dp[l][r][k];
                    if split_k < i32::MAX {
                        dp[l][r][1] = sum + dp[l][r][k];
                    }
                }
            }
        }
        let ans = dp[0][n - 1][1];
        assert!(ans < i32::MAX);
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
    #[case(vec![3,2,4,1], 2, 20)]
    #[case(vec![3,2,4,1], 3, -1)]
    #[case(vec![3,5,1,2,6], 3, 25)]
    fn case(#[case] stones: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::merge_stones(stones, k);
        assert_eq!(actual, expected);
    }
}
