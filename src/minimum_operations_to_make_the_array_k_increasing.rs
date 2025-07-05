//! Solution for https://leetcode.com/problems/minimum-operations-to-make-the-array-k-increasing
//! 2111. Minimum Operations to Make the Array K-Increasing

fn LIS(a: &Vec<i32>) -> usize {
    let mut dp = vec![i32::MAX; a.len() + 1];
    dp[0] = i32::MIN;
    for &x in a {
        let pos = dp.partition_point(|&y| y <= x);
        assert!(pos > 0);
        dp[pos] = dp[pos].min(x);
    }

    for len in 0..=a.len() {
        if dp[len] == i32::MAX {
            return len - 1;
        }
    }
    a.len()
}

impl Solution {
    pub fn k_increasing(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut ans = 0;
        for rem in 0..k {
            let mut cur = Vec::new();
            for i in (rem..a.len()).step_by(k) {
                cur.push(a[i]);
            }
            ans += cur.len() - LIS(&cur);
        }

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,4,3,2,1], 1, 4)]
    #[case(vec![4,1,5,2,6,2], 2, 0)]
    #[case(vec![4,1,5,2,6,2], 3, 2)]
    fn case(#[case] arr: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::k_increasing(arr, k);
        assert_eq!(actual, expected);
    }
}
