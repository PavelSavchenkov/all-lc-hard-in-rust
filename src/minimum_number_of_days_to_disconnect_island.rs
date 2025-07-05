//! Solution for https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island
//! 1568. Minimum Number of Days to Disconnect Island

use std::collections::VecDeque;

const di: [i32; 4] = [1, -1, 0, 0];
const dj: [i32; 4] = [0, 0, 1, -1];

fn cnt_comps(g: &Vec<Vec<i32>>) -> usize {
    let n = g.len();
    let m = g[0].len();
    let mut was = vec![vec![false; m]; n];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if !was[i][j] && g[i][j] == 1 {
                ans += 1;
                let mut q = VecDeque::new();
                q.push_back((i, j));
                was[i][j] = true;
                while !q.is_empty() {
                    let (ii, jj) = q.pop_front().unwrap();
                    for k in 0..4 {
                        let ni = ii as i32 + di[k];
                        let nj = jj as i32 + dj[k];
                        if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                            continue;
                        }
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if g[ni][nj] == 1 && !was[ni][nj] {
                            was[ni][nj] = true;
                            q.push_back((ni, nj));
                        }
                    }
                }
            }
        }
    }
    ans
}

impl Solution {
    pub fn min_days(mut g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        if cnt_comps(&g) != 1 {
            return 0;
        }

        for i in 0..n {
            for j in 0..m {
                if g[i][j] == 1 {
                    g[i][j] = 0;
                    if cnt_comps(&g) != 1 {
                        return 1;
                    }
                    g[i][j] = 1;
                }
            }
        }
        2
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,1,0],vec![0,1,1,0],vec![0,0,0,0]], 2)]
    #[case(vec![vec![1,1]], 2)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_days(grid);
        assert_eq!(actual, expected);
    }
}
