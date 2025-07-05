//! Solution for https://leetcode.com/problems/remove-boxes
//! 546. Remove Boxes

fn remax(x: &mut i32, y: i32) {
    if *x < y {
        *x = y;
    }
}

impl Solution {
    pub fn remove_boxes(mut a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();
        for x in &mut a {
            let pos = vals.binary_search(x).unwrap();
            *x = pos as i32;
        }

        let MIN = -(n as i32 * n as i32);
        let mut dp = vec![vec![0; n]; n];
        let mut dp2 = vec![vec![vec![MIN; n + 1]; n]; n];
        for len in 1..=n {
            // dp2
            for l in 0..=n - len {
                if len == 1 {
                    dp2[l][l][1] = 1;
                    continue;
                }
                let r = l + len - 1;
                for k in 1..=len {
                    if k == 1 {
                        dp2[l][r][k] = dp[l][r - 1] + 1;
                        continue;
                    }
                    for prev in l + k - 2..r {
                        if a[prev] == a[r] {
                            let ndp = dp2[l][prev][k - 1] + 2 * k as i32 - 1 + dp[prev + 1][r - 1];
                            remax(&mut dp2[l][r][k], ndp);
                        }
                    }
                }
            }

            // dp
            for l in 0..=n - len {
                let r = l + len - 1;
                for k in 1..=len {
                    for i in l + k - 1..r {
                        let ndp = dp2[l][i][k] + dp[i + 1][r];
                        remax(&mut dp[l][r], ndp);
                    }
                    remax(&mut dp[l][r], dp2[l][r][k]);
                }
            }
        }

        dp[0][n - 1] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,2,2,2,3,4,3,1], 23)]
    #[case(vec![1,1,1], 9)]
    #[case(vec![1], 1)]
    fn case(#[case] boxes: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::remove_boxes(boxes);
        assert_eq!(actual, expected);
    }
}
