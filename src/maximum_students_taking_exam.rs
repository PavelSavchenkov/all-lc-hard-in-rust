//! Solution for https://leetcode.com/problems/maximum-students-taking-exam
//! 1349. Maximum Students Taking Exam

fn bit(mask: usize, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let n = seats.len();
        let m = seats[0].len();

        let mut good_masks = vec![Vec::new(); n];
        for i in 0..n {
            for mask in 0..1 << m {
                let mut good = true;
                for j in 0..m {
                    if !bit(mask, j) {
                        continue;
                    }
                    if seats[i][j] == '#' {
                        good = false;
                        break;
                    }
                    if j > 0 && bit(mask, j - 1) {
                        good = false;
                        break;
                    }
                    if j + 1 < m && bit(mask, j + 1) {
                        good = false;
                        break;
                    }
                }
                if good {
                    good_masks[i].push(mask);
                }
            }
        }

        let mut dp = vec![-1; 1 << m];
        dp[0] = 0;
        for i in 0..n {
            let mut ndp = vec![-1; 1 << m];
            for up in 0..1 << m {
                let up_dp = dp[up];
                if up_dp < 0 {
                    continue;
                }
                for &cur in &good_masks[i] {
                    let mut good = true;
                    for j in 0..m {
                        if !bit(cur, j) {
                            continue;
                        }
                        if j > 0 && bit(up, j - 1) {
                            good = false;
                            break;
                        }
                        if j + 1 < m && bit(up, j + 1) {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        ndp[cur] = ndp[cur].max(up_dp + cur.count_ones() as i32);
                    }
                }
            }
            dp = ndp;
        }
        let ans = *dp.iter().max().unwrap();
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
    #[case(vec![vec!['#','.','#','#','.','#'],vec!['.','#','#','#','#','.'],vec!['#','.','#','#','.','#']], 4)]
    #[case(vec![vec!['.','#'],vec!['#','#'],vec!['#','.'],vec!['#','#'],vec!['.','#']], 3)]
    #[case(vec![vec!['#','.','.','.','#'],vec!['.','#','.','#','.'],vec!['.','.','#','.','.'],vec!['.','#','.','#','.'],vec!['#','.','.','.','#']], 10)]
    fn case(#[case] seats: Vec<Vec<char>>, #[case] expected: i32) {
        let actual = Solution::max_students(seats);
        assert_eq!(actual, expected);
    }
}
