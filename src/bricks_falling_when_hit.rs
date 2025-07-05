//! Solution for https://leetcode.com/problems/bricks-falling-when-hit
//! 803. Bricks Falling When Hit

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

struct Hit {
    row: usize,
    col: usize,
}

impl Solution {
    pub fn hit_bricks(g: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let n = g.len();
        let m = g[0].len();

        let bfs = |g_initial: &Vec<Vec<i32>>,
                   g: &mut Vec<Vec<i32>>,
                   is_hit: &Vec<Vec<bool>>,
                   i0: usize,
                   j0: usize|
         -> usize {
            let mut ans = 0;
            let mut q = VecDeque::new();
            q.push_front((i0, j0));
            while !q.is_empty() {
                let (i, j) = q.pop_back().unwrap();
                if g[i][j] == 1 {
                    continue;
                }
                if is_hit[i][j] {
                    continue;
                }
                if g_initial[i][j] == 0 {
                    continue;
                }
                g[i][j] = 1;
                ans += 1;
                for d in 0..4 {
                    let ni = i as i32 + di[d];
                    let nj = j as i32 + dj[d];
                    if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    q.push_back((ni, nj));
                }
            }
            ans
        };

        let hits: Vec<_> = hits
            .iter()
            .map(|h| Hit {
                row: h[0] as usize,
                col: h[1] as usize,
            })
            .collect();

        let g_initial = g.clone();
        let mut g = vec![vec![0; m]; n];
        let mut is_hit = vec![vec![false; m]; n];
        for hit in &hits {
            is_hit[hit.row][hit.col] = true;
        }

        let mut cnt_alive = 0;
        for j in 0..m {
            cnt_alive += bfs(&g_initial, &mut g, &is_hit, 0, j);
        }

        let mut ans = vec![0; hits.len()];
        for i in (0..hits.len()).rev() {
            let h = &hits[i];
            assert!(is_hit[h.row][h.col]);
            assert!(g[h.row][h.col] == 0);
            is_hit[h.row][h.col] = false;
            if g_initial[h.row][h.col] == 0 {
                continue;
            }

            let mut wake_up = false;
            for d in 0..4 {
                let ni = h.row as i32 + di[d];
                let nj = h.col as i32 + dj[d];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if g[ni][nj] == 1 {
                    wake_up = true;
                }
            }
            if h.row == 0 {
                wake_up = true;
            }
            if wake_up {
                let new_alive = bfs(&g_initial, &mut g, &is_hit, h.row, h.col);
                ans[i] = (new_alive - 1) as i32;
            }
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
    #[case(vec![vec![1,0,0,0],vec![1,1,1,0]], vec![vec![1,0]], vec![2])]
    #[case(vec![vec![1,0,0,0],vec![1,1,0,0]], vec![vec![1,1],vec![1,0]], vec![0,0])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] hits: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::hit_bricks(grid, hits);
        assert_eq!(actual, expected);
    }
}
