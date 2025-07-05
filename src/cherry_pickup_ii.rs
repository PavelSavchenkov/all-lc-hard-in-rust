//! Solution for https://leetcode.com/problems/cherry-pickup-ii
//! 1463. Cherry Pickup II

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn cherry_pickup(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut dp = vec![vec![i32::MIN; m]; m];
        dp[0][m - 1] = g[0][0] + g[0][m - 1];
        let mut ans = 0;
        for i in 1..n {
            let mut ndp = vec![vec![i32::MIN; m]; m];
            for j1 in 0..m {
                for j2 in 0..m {
                    let cur_dp = dp[j1][j2];
                    if cur_dp == i32::MIN {
                        continue;
                    }
                    for nj1 in (j1 as i32 - 1).max(0) as usize..(j1 + 2).min(m) {
                        for nj2 in (j2 as i32 - 1).max(0) as usize..(j2 + 2).min(m) {
                            let mut upd = g[i][nj1];
                            if nj2 != nj1 {
                                upd += g[i][nj2];
                            }
                            upd += cur_dp;
                            remax(&mut ndp[nj1][nj2], upd);
                            if i + 1 == n {
                                ans = ans.max(upd);
                            }
                        }
                    }
                }
            }
            dp = ndp;
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
    #[case(vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]], 24)]
    #[case(vec![vec![1,0,0,0,0,0,1],vec![2,0,0,0,0,3,0],vec![2,0,9,0,0,0,0],vec![0,3,0,5,4,0,0],vec![1,0,2,3,0,0,6]], 28)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::cherry_pickup(grid);
        assert_eq!(actual, expected);
    }
}
