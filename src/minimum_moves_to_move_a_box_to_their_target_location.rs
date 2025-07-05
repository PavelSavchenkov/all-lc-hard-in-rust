//! Solution for https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location
//! 1263. Minimum Moves to Move a Box to Their Target Location

use std::collections::VecDeque;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn min_push_box(g: Vec<Vec<char>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let try_go_point = |i: usize, j: usize, dir: usize| -> Option<(usize, usize)> {
            let ni = i as i32 + di[dir];
            let nj = j as i32 + dj[dir];
            if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                return None;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if g[ni][nj] == '#' {
                return None;
            }
            Some((ni as usize, nj as usize))
        };

        let try_go = |p: (usize, usize),
                      b: (usize, usize),
                      dir: usize|
         -> Option<((usize, usize), (usize, usize))> {
            let np = try_go_point(p.0, p.1, dir);
            if np.is_none() {
                return None;
            }
            let np = np.unwrap();
            let mut nb = b;
            if np == b {
                let try_nb = try_go_point(b.0, b.1, dir);
                if try_nb.is_none() {
                    return None;
                }
                nb = try_nb.unwrap();
            }
            Some((np, nb))
        };

        let mut dp = vec![vec![usize::MAX; n * m]; n * m];
        let mut s_init = n * m;
        let mut b_init = n * m;
        let mut T = n * m;
        for i in 0..n {
            for j in 0..m {
                // match...
                if g[i][j] == 'S' {
                    s_init = i * m + j;
                } else if g[i][j] == 'B' {
                    b_init = i * m + j;
                } else if g[i][j] == 'T' {
                    T = i * m + j;
                }
            }
        }
        assert!(T < n * m);
        let mut q = VecDeque::new();
        q.push_back((s_init, b_init));
        dp[s_init][b_init] = 0;
        let mut ans = usize::MAX;
        while !q.is_empty() {
            let (s, b) = q.pop_front().unwrap();
            let cur_dp = dp[s][b];
            if b == T {
                ans = ans.min(cur_dp);
                continue;
            }
            for dir in 0..4 {
                let next = try_go((s / m, s % m), (b / m, b % m), dir);
                if next.is_none() {
                    continue;
                }
                let (ns, nb) = next.unwrap();
                let ns = ns.0 * m + ns.1;
                let nb = nb.0 * m + nb.1;
                let mut ndp = cur_dp;
                if nb != b {
                    ndp += 1;
                }
                if dp[ns][nb] > ndp {
                    dp[ns][nb] = ndp;
                    q.push_back((ns, nb));
                }
            }
        }
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
    #[case(vec![vec!['#','#','#','#','#','#'],vec!['#','T','#','#','#','#'],vec!['#','.','.','B','.','#'],vec!['#','.','#','#','.','#'],vec!['#','.','.','.','S','#'],vec!['#','#','#','#','#','#']], 3)]
    #[case(vec![vec!['#','#','#','#','#','#'],vec!['#','T','#','#','#','#'],vec!['#','.','.','B','.','#'],vec!['#','#','#','#','.','#'],vec!['#','.','.','.','S','#'],vec!['#','#','#','#','#','#']], -1)]
    #[case(vec![vec!['#','#','#','#','#','#'],vec!['#','T','.','.','#','#'],vec!['#','.','#','B','.','#'],vec!['#','.','.','.','.','#'],vec!['#','.','.','.','S','#'],vec!['#','#','#','#','#','#']], 5)]
    fn case(#[case] grid: Vec<Vec<char>>, #[case] expected: i32) {
        let actual = Solution::min_push_box(grid);
        assert_eq!(actual, expected);
    }
}
