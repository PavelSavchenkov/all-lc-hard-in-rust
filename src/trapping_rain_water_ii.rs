//! Solution for https://leetcode.com/problems/trapping-rain-water-ii
//! 407. Trapping Rain Water II

use std::collections::BTreeSet;

const di: [i32; 4] = [0, 0, 1, -1];
const dj: [i32; 4] = [1, -1, 0, 0];

impl Solution {
    pub fn trap_rain_water(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();

        // minimal max on path to the border
        let mut dp = vec![vec![i32::MAX; m]; n];
        let mut set = BTreeSet::new();
        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0 || i + 1 == n || j + 1 == m {
                    dp[i][j] = a[i][j];
                    set.insert((dp[i][j], (i, j)));
                }
            }
        }
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
                let ndp = dp[i][j].max(a[ni][nj]);
                if ndp < dp[ni][nj] {
                    set.remove(&(dp[ni][nj], (ni, nj)));
                    dp[ni][nj] = ndp;
                    set.insert((dp[ni][nj], (ni, nj)));
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += dp[i][j] - a[i][j];
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
    #[case(vec![vec![1,4,3,1,3,2],vec![3,2,1,3,2,4],vec![2,3,3,2,3,1]], 4)]
    #[case(vec![vec![3,3,3,3,3],vec![3,2,2,2,3],vec![3,2,1,2,3],vec![3,2,2,2,3],vec![3,3,3,3,3]], 10)]
    fn case(#[case] height_map: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::trap_rain_water(height_map);
        assert_eq!(actual, expected);
    }
}
