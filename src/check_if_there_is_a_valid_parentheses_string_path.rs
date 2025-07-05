//! Solution for https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path
//! 2267.  Check if There Is a Valid Parentheses String Path

impl Solution {
    pub fn has_valid_path(g: Vec<Vec<char>>) -> bool {
        let g: Vec<Vec<i32>> = g
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| if c as u8 == b'(' { 1 } else { -1 })
                    .collect()
            })
            .collect();

        let n = g.len();
        let m = g[0].len();
        let B = (n + m) / 2;

        if g[0][0] != 1 || g[n - 1][m - 1] != -1 {
            return false;
        }

        let mut dp = vec![vec![vec![false; B + 1]; m]; n];
        dp[0][0][1] = true;
        for i in 0..n {
            for j in 0..m {
                for b in 0..=B {
                    if !dp[i][j][b] {
                        continue;
                    }
                    for (di, dj) in [(1, 0), (0, 1)] {
                        let ni = i + di;
                        if ni >= n {
                            continue;
                        }
                        let nj = j + dj;
                        if nj >= m {
                            continue;
                        }
                        let new_b = b as i32 + g[ni][nj];
                        if new_b < 0 || new_b > B as i32 {
                            continue;
                        }
                        dp[ni][nj][(b as i32 + g[ni][nj]) as usize] = true
                    }
                }
            }
        }

        dp[n - 1][m - 1][0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['(','(','('],vec![')','(',')'],vec!['(','(',')'],vec!['(','(',')']], true)]
    #[case(vec![vec![')',')'],vec!['(','(']], false)]
    fn case(#[case] grid: Vec<Vec<char>>, #[case] expected: bool) {
        let actual = Solution::has_valid_path(grid);
        assert_eq!(actual, expected);
    }
}
