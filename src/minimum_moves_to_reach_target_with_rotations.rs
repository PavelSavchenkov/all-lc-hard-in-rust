//! Solution for https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations
//! 1210. Minimum Moves to Reach Target with Rotations

fn remin(x: &mut usize, y: usize) {
    if *x > y {
        *x = y;
    }
}

impl Solution {
    pub fn minimum_moves(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let checked_pos =
            |i: usize, j: usize, orientation: usize| -> Option<(usize, usize, usize)> {
                if i >= n || j >= m {
                    return None;
                }
                if g[i][j] == 1 {
                    return None;
                }
                if orientation == 0 {
                    if j + 1 >= m || g[i][j + 1] == 1 {
                        return None;
                    }
                } else {
                    if i + 1 >= n || g[i + 1][j] == 1 {
                        return None;
                    }
                }
                Some((i, j, orientation))
            };

        let try_move_to =
            |i: usize, j: usize, orientation: usize, dir: usize| -> Option<(usize, usize, usize)> {
                match dir {
                    0 => return checked_pos(i, j + 1, orientation),
                    1 => return checked_pos(i + 1, j, orientation),
                    2 => {
                        if orientation == 1 {
                            return None;
                        }
                        if i + 1 >= n || g[i + 1][j + 1] == 1 {
                            return None;
                        }
                        return checked_pos(i, j, 1 - orientation);
                    }
                    3 => {
                        if orientation == 0 {
                            return None;
                        }
                        if j + 1 >= m || g[i + 1][j + 1] == 1 {
                            return None;
                        }
                        return checked_pos(i, j, 1 - orientation);
                    }
                    _ => panic!("Wrong dir = {}", dir),
                }
            };

        let mut dp = vec![vec![vec![usize::MAX; 2]; m]; n];
        dp[0][0][0] = 0;
        for i in 0..n {
            for j in 0..m {
                let mut range: Vec<usize> = (0..2).collect();
                for iter in 0..2 {
                    for &orientation in &range {
                        let cur_dp = dp[i][j][orientation];
                        if cur_dp == usize::MAX {
                            continue;
                        }
                        for dir in 0..4 {
                            if let Some((ni, nj, no)) = try_move_to(i, j, orientation, dir) {
                                remin(&mut dp[ni][nj][no], cur_dp + 1);
                            }
                        }
                    }
                    range.reverse();
                }
            }
        }

        let ans = dp[n - 1][m - 2][0];
        if ans == usize::MAX {
            return -1;
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
    #[case(todo!("[[0,0,0,0,0,1],[1,1,0,0,1,0],[0,0,0,0,1,1],[0,0,1,0,1,0],[0,1,1,0,0,0],[0,1,1,0,0,0]]\r"), 11)]
    #[case(todo!("[[0,0,1,1,1,1],[0,0,0,0,1,1],[1,1,0,0,0,1],[1,1,1,0,0,1],[1,1,1,0,0,1],[1,1,1,0,0,0]]\r"), 9)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_moves(grid);
        assert_eq!(actual, expected);
    }
}
