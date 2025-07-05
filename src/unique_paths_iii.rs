//! Solution for https://leetcode.com/problems/unique-paths-iii
//! 980. Unique Paths III

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn unique_paths_iii(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dp = vec![0; n * m * (1 << (n * m))];
        let C = n * m;
        for i in 0..n {
            for j in 0..m {
                if g[i][j] == 1 {
                    let mask = 1 << (i * m + j);
                    dp[mask * C + i * m + j] = 1;
                }
            }
        }

        let mut next_pos_mask = vec![0; n * m];
        for i in 0..n {
            for j in 0..m {
                if g[i][j] == -1 {
                    continue;
                }
                let pos = i * m + j;
                let mut mask = 0;
                for d in 0..4 {
                    let ni = i as i32 + di[d];
                    let nj = j as i32 + dj[d];
                    if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    let npos = ni * m + nj;
                    if g[ni][nj] == -1 {
                        continue;
                    }
                    mask |= 1 << npos;
                }
                next_pos_mask[pos] = mask;
            }
        }

        let mut masks: Vec<usize> = (0..(1 << (n * m))).collect();
        masks.sort_by_key(|&m| m.count_ones());
        for &mask in &masks {
            let mut mask_iter = mask;
            while mask_iter > 0 {
                let pos = mask_iter.trailing_zeros() as usize;
                mask_iter ^= 1 << pos;
                let cur_dp = dp[mask * C + pos];
                if cur_dp == 0 {
                    continue;
                }
                let mut mask_next_iter = next_pos_mask[pos] & !mask;
                while mask_next_iter > 0 {
                    let npos = mask_next_iter.trailing_zeros() as usize;
                    mask_next_iter ^= 1 << npos;
                    let nmask = mask | 1 << npos;
                    dp[nmask * C + npos] += cur_dp;
                }
            }
        }

        let mut full_mask = 0;
        for i in 0..n {
            for j in 0..m {
                if g[i][j] != -1 {
                    full_mask |= 1 << (i * m + j);
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if g[i][j] == 2 {
                    return dp[full_mask * C + i * m + j];
                }
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,2,-1]], 2)]
    #[case(vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,2]], 4)]
    #[case(vec![vec![0,1],vec![2,0]], 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::unique_paths_iii(grid);
        assert_eq!(actual, expected);
    }
}
