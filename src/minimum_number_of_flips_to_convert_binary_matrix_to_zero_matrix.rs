//! Solution for https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix
//! 1284. Minimum Number of Flips to Convert Binary Matrix to Zero Matrix

use std::collections::VecDeque;

const di: [i32; 5] = [0, 0, 1, -1, 0];
const dj: [i32; 5] = [1, -1, 0, 0, 0];

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        let mut mask_init = 0;
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 {
                    mask_init |= 1 << (i * m + j);
                }
            }
        }

        let mut dp = vec![usize::MAX; 1 << (n * m)];
        let mut q = VecDeque::new();
        dp[mask_init] = 0;
        q.push_back(mask_init);
        while !q.is_empty() {
            let mask = q.pop_front().unwrap();
            let cur_dp = dp[mask];
            for i in 0..n {
                for j in 0..m {
                    let mut nmask = mask;
                    for d in 0..5 {
                        let ni = i as i32 + di[d];
                        let nj = j as i32 + dj[d];
                        if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                            continue;
                        }
                        nmask ^= 1 << (ni as usize * m + nj as usize);
                    }
                    if dp[nmask] > cur_dp + 1 {
                        dp[nmask] = cur_dp + 1;
                        q.push_back(nmask);
                    }
                }
            }
        }

        let ans = dp[0];
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
    #[case(vec![vec![0,0],vec![0,1]], 3)]
    #[case(vec![vec![0]], 0)]
    #[case(vec![vec![1,0,0],vec![1,0,0]], -1)]
    fn case(#[case] mat: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_flips(mat);
        assert_eq!(actual, expected);
    }
}
