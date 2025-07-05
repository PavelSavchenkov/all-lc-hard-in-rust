//! Solution for https://leetcode.com/problems/longest-increasing-path-in-a-matrix
//! 329. Longest Increasing Path in a Matrix

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn longest_increasing_path(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();

        let mut ij = Vec::with_capacity(n * m);
        for i in 0..n {
            for j in 0..m {
                ij.push((i, j));
            }
        }
        ij.sort_by_key(|&(i, j)| a[i][j]);

        let mut ans = 1;
        let mut dp = vec![vec![1; m]; n];
        for &(i, j) in &ij {
            for k in 0..4 {
                let ni = i as i32 + di[k];
                let nj = j as i32 + dj[k];
                if !(0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32) {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if a[ni][nj] <= a[i][j] {
                    continue;
                }
                let ndp = dp[i][j] + 1;
                dp[ni][nj] = dp[ni][nj].max(ndp);
                ans = ans.max(ndp);
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
    #[case(vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]], 4)]
    #[case(vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]], 4)]
    #[case(vec![vec![1]], 1)]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::longest_increasing_path(matrix);
        assert_eq!(actual, expected);
    }
}
