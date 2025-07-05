//! Solution for https://leetcode.com/problems/maximal-rectangle
//! 85. Maximal Rectangle

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let a: Vec<_> = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| if c == '0' { 0 } else { 1 })
                    .collect::<Vec<i32>>()
            })
            .collect();
        let n = a.len();
        let m = a[0].len();

        let mut ans = 0;
        for i in 0..n {
            let mut s = vec![1; m];
            for i2 in i..n {
                for j in 0..m {
                    s[j] &= a[i2][j];
                }
                let mut prev0 = -1;
                let mut max_w = 0;
                for j in 0..m {
                    if s[j] == 1 {
                        let len = (j as i32 - prev0) as usize;
                        max_w = max_w.max(len);
                    } else {
                        prev0 = j as i32;
                    }
                }
                ans = ans.max(max_w * (i2 - i + 1));
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
    #[case(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']], 6)]
    #[case(vec![vec!['0']], 0)]
    #[case(vec![vec!['1']], 1)]
    fn case(#[case] matrix: Vec<Vec<char>>, #[case] expected: i32) {
        let actual = Solution::maximal_rectangle(matrix);
        assert_eq!(actual, expected);
    }
}
