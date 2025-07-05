//! Solution for https://leetcode.com/problems/minimum-total-distance-traveled
//! 2463. Minimum Total Distance Traveled

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();
        factory.sort();

        let n = robot.len();
        let m = factory.len();

        let mut dp = vec![vec![i64::MAX; m + 1]; n + 1];
        dp[0][0] = 0;
        for i in 0..n {
            for j in 0..m {
                let cur_dp = dp[i][j];
                if cur_dp == i64::MAX {
                    continue;
                }
                let factory_pos = factory[j][0];
                let limit = factory[j][1] as usize;
                let mut sum_len: i64 = 0;
                for cnt in 0..=(n - i).min(limit) {
                    if cnt > 0 {
                        sum_len += (robot[i + cnt - 1] - factory_pos).abs() as i64;
                    }
                    dp[i + cnt][j + 1] = dp[i + cnt][j + 1].min(cur_dp + sum_len);
                }
            }
        }
        *dp[n].iter().min().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,4,6], vec![vec![2,2],vec![6,2]], 4)]
    #[case(vec![1,-1], vec![vec![-2,1],vec![2,1]], 2)]
    fn case(#[case] robot: Vec<i32>, #[case] factory: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::minimum_total_distance(robot, factory);
        assert_eq!(actual, expected);
    }
}
