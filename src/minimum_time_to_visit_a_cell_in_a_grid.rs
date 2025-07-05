//! Solution for https://leetcode.com/problems/minimum-time-to-visit-a-cell-in-a-grid
//! 2577. Minimum Time to Visit a Cell In a Grid

use std::collections::BTreeSet;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn minimum_time(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        if g[0][1] > 1 && g[1][0] > 1 {
            return -1;
        }

        let mut dp = vec![vec![i32::MAX; m]; n];
        dp[0][0] = 0;
        let mut set = BTreeSet::new();
        set.insert((0, (0, 0)));
        while !set.is_empty() {
            let (_, (i, j)) = set.pop_first().unwrap();
            for k in 0..4 {
                let ni = i as i32 + di[k];
                let nj = j as i32 + dj[k];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                let mut ndp = dp[i][j] + 1;
                if g[ni][nj] > ndp {
                    let mut after = g[ni][nj];
                    if after % 2 != ndp % 2 {
                        after += 1;
                    }
                    ndp = after;
                }
                if dp[ni][nj] <= ndp {
                    continue;
                }
                set.remove(&(dp[ni][nj], (ni, nj)));
                dp[ni][nj] = ndp;
                set.insert((dp[ni][nj], (ni, nj)));
            }
        }

        let ans = dp[n - 1][m - 1];
        if ans == i32::MAX {
            return -1;
        }
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
    #[case(vec![vec![0,1,3,2],vec![5,1,2,5],vec![4,3,8,6]], 7)]
    #[case(vec![vec![0,2,4],vec![3,2,1],vec![1,0,4]], -1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_time(grid);
        assert_eq!(actual, expected);
    }
}
