//! Solution for https://leetcode.com/problems/maximum-strictly-increasing-cells-in-a-matrix
//! 2713. Maximum Strictly Increasing Cells in a Matrix

impl Solution {
    pub fn max_increasing_cells(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();

        let mut vals = Vec::new();
        for i in 0..n {
            for j in 0..m {
                vals.push((a[i][j], (i, j)));
            }
        }
        vals.sort();
        vals.reverse();

        let mut global_ans = 0;
        let mut row_max = vec![0; n];
        let mut col_max = vec![0; m];
        let mut l = 0;
        while l < vals.len() {
            let mut r = l;
            while r < vals.len() && vals[l].0 == vals[r].0 {
                r += 1;
            }

            let mut answers = Vec::new();
            for k in l..r {
                let (_, (i, j)) = vals[k];
                let answer = row_max[i].max(col_max[j]);
                answers.push(answer + 1);
            }
            for k in l..r {
                let (_, (i, j)) = vals[k];
                let answer = answers[k - l];
                row_max[i] = row_max[i].max(answer);
                col_max[j] = col_max[j].max(answer);
                global_ans = global_ans.max(answer);
            }

            l = r;
        }
        global_ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![3,1],vec![3,4]], 2)]
    #[case(vec![vec![1,1],vec![1,1]], 1)]
    #[case(vec![vec![3,1,6],vec![-9,5,7]], 4)]
    fn case(#[case] mat: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_increasing_cells(mat);
        assert_eq!(actual, expected);
    }
}
