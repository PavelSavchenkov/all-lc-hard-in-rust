//! Solution for https://leetcode.com/problems/cherry-pickup
//! 741. Cherry Pickup

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn cherry_pickup(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dp = vec![vec![vec![-1; m]; m]; n + m - 1];
        dp[0][0][0] = g[0][0];
        for diag in 0..n + m - 2 {
            for c0 in 0..m.min(diag + 1) {
                let r0 = diag - c0;
                if r0 >= n {
                    continue;
                }
                for c1 in 0..m.min(diag + 1) {
                    let r1 = diag - c1;
                    if r1 >= n {
                        continue;
                    }
                    let cur_dp = dp[diag][c0][c1];
                    if cur_dp < 0 {
                        continue;
                    }

                    for new_c0 in [c0, c0 + 1] {
                        if new_c0 >= m {
                            continue;
                        }
                        for new_c1 in [c1, c1 + 1] {
                            if new_c1 >= m {
                                continue;
                            }
                            let new_r0 = diag + 1 - new_c0;
                            if new_r0 >= n {
                                continue;
                            }
                            let new_r1 = diag + 1 - new_c1;
                            if new_r1 >= n {
                                continue;
                            }
                            let mut new_dp = cur_dp;
                            if g[new_r0][new_c0] == -1 || g[new_r1][new_c1] == -1 {
                                continue;
                            }
                            new_dp += g[new_r0][new_c0];
                            if new_r0 != new_r1 {
                                new_dp += g[new_r1][new_c1];
                            }
                            remax(&mut dp[diag + 1][new_c0][new_c1], new_dp);
                        }
                    }
                }
            }
        }

        dp[n + m - 2][m - 1][m - 1].max(0)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,-1],vec![1,0,-1],vec![1,1,1]], 5)]
    #[case(vec![vec![1,1,-1],vec![1,-1,1],vec![-1,1,1]], 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::cherry_pickup(grid);
        assert_eq!(actual, expected);
    }
}
