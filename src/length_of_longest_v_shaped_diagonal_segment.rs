//! Solution for https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment
//! 3459. Length of Longest V-Shaped Diagonal Segment

const di: [i32; 4] = [1, -1, 1, -1];
const dj: [i32; 4] = [-1, 1, 1, -1];

fn next(x: i32) -> i32 {
    match x {
        1 => 2,
        2 => 0,
        0 => 2,
        _ => panic!(),
    }
}

fn next_dir(d: usize) -> usize {
    match d {
        0 => 3,
        1 => 2,
        2 => 0,
        3 => 1,
        _ => panic!(),
    }
}

impl Solution {
    pub fn len_of_v_diagonal(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dp0 = vec![vec![vec![1; 4]; m]; n];

        let mut relax_dp0 = |i: usize, j: usize, d: usize| {
            let ni = i as i32 + di[d];
            let nj = j as i32 + dj[d];
            if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                return;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if g[ni][nj] != next(g[i][j]) {
                return;
            }
            dp0[i][j][d] = 1 + dp0[ni][nj][d];
        };

        // dirs 1 and 2
        for j in (0..m).rev() {
            for i in 0..n {
                for d in [1, 2] {
                    relax_dp0(i, j, d);
                }
            }
        }
        // dirs 0 and 3
        for j in 0..m {
            for i in 0..n {
                for d in [0, 3] {
                    relax_dp0(i, j, d);
                }
            }
        }

        let mut dp1 = vec![vec![vec![1; 4]; m]; n];

        let mut relax_dp1 = |i: usize, j: usize, d: usize| {
            let mut cur_dp = 1;
            // turn
            let nd = next_dir(d);
            cur_dp = cur_dp.max(dp0[i][j][nd]);
            // try do not turn
            let ni = i as i32 + di[d];
            let nj = j as i32 + dj[d];
            if 0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if g[ni][nj] == next(g[i][j]) {
                    cur_dp = cur_dp.max(1 + dp1[ni][nj][d]);
                }
            }
            dp1[i][j][d] = dp1[i][j][d].max(cur_dp);
        };

        // dirs 1 and 2
        for j in (0..m).rev() {
            for i in 0..n {
                for d in [1, 2] {
                    relax_dp1(i, j, d);
                }
            }
        }
        // dirs 0 and 3
        for j in 0..m {
            for i in 0..n {
                for d in [0, 3] {
                    relax_dp1(i, j, d);
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if g[i][j] == 1 {
                    for d in 0..4 {
                        ans = ans.max(dp1[i][j][d]);
                    }
                }
            }
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
    #[case(vec![vec![2,2,1,2,2],vec![2,0,2,2,0],vec![2,0,1,1,0],vec![1,0,2,2,2],vec![2,0,0,2,2]], 5)]
    #[case(vec![vec![2,2,2,2,2],vec![2,0,2,2,0],vec![2,0,1,1,0],vec![1,0,2,2,2],vec![2,0,0,2,2]], 4)]
    #[case(vec![vec![1,2,2,2,2],vec![2,2,2,2,0],vec![2,0,0,0,0],vec![0,0,2,2,2],vec![2,0,0,2,0]], 5)]
    #[case(vec![vec![1]], 1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::len_of_v_diagonal(grid);
        assert_eq!(actual, expected);
    }
}
