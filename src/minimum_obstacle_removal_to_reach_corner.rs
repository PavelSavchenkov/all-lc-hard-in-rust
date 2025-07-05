//! Solution for https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner
//! 2290. Minimum Obstacle Removal to Reach Corner

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn minimum_obstacles(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dist = vec![vec![i32::MAX; m]; n];
        dist[0][0] = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            assert!(dist[i][j] < i32::MAX);
            for k in 0..4 {
                let ni = i as i32 + di[k];
                let nj = j as i32 + dj[k];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                let ndist = dist[i][j] + g[ni][nj];
                if dist[ni][nj] <= ndist {
                    continue;
                }
                dist[ni][nj] = ndist;
                q.push_back((ni, nj));
            }
        }

        dist[n - 1][m - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,1],vec![1,1,0],vec![1,1,0]], 2)]
    #[case(vec![vec![0,1,0,0,0],vec![0,1,0,1,0],vec![0,0,0,1,0]], 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_obstacles(grid);
        assert_eq!(actual, expected);
    }
}
