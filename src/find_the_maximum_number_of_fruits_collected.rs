//! Solution for https://leetcode.com/problems/find-the-maximum-number-of-fruits-collected
//! 3363. Find the Maximum Number of Fruits Collected

fn solve(mut a: Vec<Vec<i32>>) -> i32 {
    let n = a.len();
    assert!(a[0].len() == n);

    for i in 0..n {
        a[i][i] = 0;
    }

    let mut dp = vec![i32::MIN; n];
    dp[n - 1] = a[n - 1][0];

    for j in 1..n {
        let mut ndp = vec![i32::MIN; n];
        for prev_i in 0..n {
            let prev_dp = dp[prev_i];
            if prev_dp == i32::MIN {
                continue;
            }
            ndp[prev_i] = ndp[prev_i].max(prev_dp + a[prev_i][j]);
            if prev_i > 0 {
                ndp[prev_i - 1] = ndp[prev_i - 1].max(prev_dp + a[prev_i - 1][j]);
            }
            if prev_i + 1 < n {
                ndp[prev_i + 1] = ndp[prev_i + 1].max(prev_dp + a[prev_i + 1][j]);
            }
        }
        dp = ndp;
    }

    dp[n - 1]
}

impl Solution {
    pub fn max_collected_fruits(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        assert!(a[0].len() == n);

        let mut ans = 0;
        for i in 0..n {
            ans += a[i][i];
        }

        let mut b = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                b[i][j] = a[j][i];
            }
        }
        ans += solve(a);
        ans += solve(b);
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
    #[case(vec![vec![1,2,3,4],vec![5,6,8,7],vec![9,10,11,12],vec![13,14,15,16]], 100)]
    #[case(vec![vec![1,1],vec![1,1]], 4)]
    fn case(#[case] fruits: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_collected_fruits(fruits);
        assert_eq!(actual, expected);
    }
}
