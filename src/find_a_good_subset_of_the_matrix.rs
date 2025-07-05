//! Solution for https://leetcode.com/problems/find-a-good-subset-of-the-matrix
//! 2732. Find a Good Subset of the Matrix

impl Solution {
    pub fn good_subsetof_binary_matrix(g: Vec<Vec<i32>>) -> Vec<i32> {
        let n = g.len();
        let m = g[0].len();

        let mut who = vec![n; 1 << m];
        for i in 0..n {
            let mut mask = 0;
            for j in 0..m {
                mask = mask * 2 + g[i][j];
            }
            who[mask as usize] = i;
        }

        if who[0] != n {
            return vec![who[0] as i32];
        }

        for m0 in 1..1 << m {
            for m1 in 1..1 << m {
                if (m0 & m1) == 0 && who[m0] != n && who[m1] != n {
                    let mut ans = vec![who[m0] as i32, who[m1] as i32];
                    ans.sort();
                    return ans;
                }
            }
        }

        Vec::new()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,1,0],vec![0,0,0,1],vec![1,1,1,1]], vec![0,1])]
    #[case(vec![vec![0]], vec![0])]
    #[case(vec![vec![1,1,1],vec![1,1,1]], vec![])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::good_subsetof_binary_matrix(grid);
        assert_eq!(actual, expected);
    }
}
