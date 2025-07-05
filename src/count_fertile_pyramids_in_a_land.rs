//! Solution for https://leetcode.com/problems/count-fertile-pyramids-in-a-land
//! 2088. Count Fertile Pyramids in a Land

impl Solution {
    pub fn count_pyramids(mut g: Vec<Vec<i32>>) -> i32 {
        let rows = g.len();
        let cols = g[0].len();

        let mut pref = vec![vec![0; cols + 1]; rows];
        for i in 0..rows {
            pref[i][0] = 0;
            for j in 0..cols {
                pref[i][j + 1] = pref[i][j] + g[i][j] as usize;
            }
        }

        let mut ans = 0;
        for it in 0..2 {
            for i in 0..rows {
                for j in 0..cols {
                    for i2 in i..rows {
                        let cnt = (i2 - i + 1) * 2 - 1;
                        if j < cnt / 2 {
                            break;
                        }
                        let l = j - cnt / 2;
                        let r = j + cnt / 2 + 1;
                        if r > cols {
                            break;
                        }
                        if pref[i2][r] - pref[i2][l] == cnt {
                            if cnt > 1 {
                                ans += 1;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            g.reverse();
            pref.reverse();
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
    #[case(vec![vec![0,1,1,0],vec![1,1,1,1]], 2)]
    #[case(vec![vec![1,1,1],vec![1,1,1]], 2)]
    #[case(vec![vec![1,1,1,1,0],vec![1,1,1,1,1],vec![1,1,1,1,1],vec![0,1,0,0,1]], 13)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_pyramids(grid);
        assert_eq!(actual, expected);
    }
}
