//! Solution for https://leetcode.com/problems/escape-the-spreading-fire
//! 2258. Escape the Spreading Fire

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, -1, 1];
const dj: [i32; 4] = [-1, 1, 0, 0];

const FIRE: i32 = 1;
const WALL: i32 = 2;

impl Solution {
    pub fn maximum_minutes(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let on = |i: i32, j: i32| -> bool { 0 <= i && i < n as i32 && 0 <= j && j < m as i32 };

        let dist_fire = {
            let mut d = vec![vec![i32::MAX; m]; n];
            let mut q = VecDeque::new();
            for i in 0..n {
                for j in 0..m {
                    if g[i][j] == FIRE {
                        d[i][j] = 0;
                        q.push_back((i, j));
                    }
                }
            }
            while !q.is_empty() {
                let (i, j) = q.pop_front().unwrap();
                for k in 0..4 {
                    let ni = i as i32 + di[k];
                    let nj = j as i32 + dj[k];
                    if !on(ni, nj) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if g[ni][nj] == WALL {
                        continue;
                    }
                    if d[ni][nj] < d[i][j] + 1 {
                        continue;
                    }
                    d[ni][nj] = d[i][j] + 1;
                    q.push_back((ni, nj));
                }
            }
            d
        };

        let can_reach_safe_house = |T: i32| -> bool {
            let mut d = vec![vec![i32::MAX; m]; n];
            d[0][0] = T;
            let mut q = VecDeque::new();
            q.push_back((0 as usize, 0 as usize));
            while !q.is_empty() {
                let (i, j) = q.pop_front().unwrap();
                let when_fire = dist_fire[i][j];
                if when_fire < d[i][j] {
                    continue;
                }
                if i == n - 1 && j == m - 1 {
                    return true;
                }
                if when_fire == d[i][j] {
                    continue;
                }
                for k in 0..4 {
                    let ni = i as i32 + di[k];
                    let nj = j as i32 + dj[k];
                    if !on(ni, nj) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if g[ni][nj] == WALL {
                        continue;
                    }
                    if d[ni][nj] < d[i][j] + 1 {
                        continue;
                    }
                    d[ni][nj] = d[i][j] + 1;
                    q.push_back((ni, nj));
                }
            }

            false
        };

        if !can_reach_safe_house(0) {
            return -1;
        }

        const INF: i32 = 1_000_000_000;
        let mut L = 0;
        let mut R = INF + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can_reach_safe_house(M) {
                L = M
            } else {
                R = M
            }
        }

        L
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,2,0,0,0,0,0],vec![0,0,0,2,2,1,0],vec![0,2,0,0,1,2,0],vec![0,0,2,2,2,0,2],vec![0,0,0,0,0,0,0]], 3)]
    #[case(vec![vec![0,0,0,0],vec![0,1,2,0],vec![0,2,0,0]], -1)]
    #[case(vec![vec![0,0,0],vec![2,2,0],vec![1,2,0]], 1000000000)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximum_minutes(grid);
        assert_eq!(actual, expected);
    }
}
