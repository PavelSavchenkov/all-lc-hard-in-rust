//! Solution for https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points
//! 1595. Minimum Cost to Connect Two Groups of Points

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();

        let mut min_cost = vec![vec![i32::MAX; m + 1]; n];
        for i in 0..n {
            for j in 0..m {
                min_cost[i][j + 1] = min_cost[i][j].min(cost[i][j]);
            }
        }

        // first_uncovered_on_right, mask_left,
        let mut dp = vec![vec![i32::MAX; 1 << n]; m + 1];
        dp[0][0] = 0;
        for i in 0..=m {
            for mask in 0..((1 << n) as usize) {
                let cur_dp = dp[i][mask];
                if cur_dp == i32::MAX {
                    continue;
                }
                // cover i-th on the right
                if i < m {
                    for j in 0..n {
                        let nmask = mask | (1 << j);
                        let ndp = cur_dp + cost[j][i];
                        remin(&mut dp[i + 1][nmask], ndp);
                    }
                }
                // cover any one uncovered on the left
                if i > 0 {
                    for j in 0..n {
                        if ((mask >> j) & 1) == 0 {
                            let nmask = mask | (1 << j);
                            let ndp = cur_dp + min_cost[j][i];
                            remin(&mut dp[i][nmask], ndp);
                        }
                    }
                }
            }
        }
        let ans = dp[m][(1 << n) - 1];
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
    #[case(vec![vec![15,96],vec![36,2]], 17)]
    #[case(vec![vec![1,3,5],vec![4,1,1],vec![1,5,3]], 4)]
    #[case(vec![vec![2,5,1],vec![3,4,7],vec![8,1,2],vec![6,2,4],vec![3,8,8]], 10)]
    fn case(#[case] cost: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::connect_two_groups(cost);
        assert_eq!(actual, expected);
    }
}
