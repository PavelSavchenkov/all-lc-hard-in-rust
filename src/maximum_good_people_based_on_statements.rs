//! Solution for https://leetcode.com/problems/maximum-good-people-based-on-statements
//! 2151. Maximum Good People Based on Statements

impl Solution {
    pub fn maximum_good(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();

        let mut ans = 0;
        for mask in 0..(1 << n) as usize {
            let mut ok = true;
            for i in 0..n {
                if ((mask >> i) & 1) == 0 {
                    continue;
                }
                for j in 0..n {
                    if g[i][j] == 0 {
                        if ((mask >> j) & 1) == 1 {
                            ok = false;
                            break;
                        }
                    } else if g[i][j] == 1 {
                        if ((mask >> j) & 1) == 0 {
                            ok = false;
                            break;
                        }
                    }
                }
            }
            if ok {
                ans = ans.max(mask.count_ones());
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
    #[case(vec![vec![2,1,2],vec![1,2,2],vec![2,0,2]], 2)]
    #[case(vec![vec![2,0],vec![0,2]], 1)]
    fn case(#[case] statements: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximum_good(statements);
        assert_eq!(actual, expected);
    }
}
