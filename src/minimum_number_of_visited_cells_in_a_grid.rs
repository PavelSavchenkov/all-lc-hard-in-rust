//! Solution for https://leetcode.com/problems/minimum-number-of-visited-cells-in-a-grid
//! 2617. Minimum Number of Visited Cells in a Grid

#[derive(Clone)]
struct FenwTreeMinSuff {
    t: Vec<i64>,
}

impl FenwTreeMinSuff {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        assert!(pos < self.t.len());
        let mut i = self.t.len() - pos - 1;
        while i < self.t.len() {
            self.t[i] = self.t[i].min(val);
            i |= i + 1;
        }
    }

    // [r, t.len())
    fn get_min(&mut self, r: usize) -> i64 {
        let mut ans = i64::MAX;
        if r == self.t.len() {
            return ans;
        }
        let mut i = self.t.len() - r - 1;
        loop {
            ans = ans.min(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        ans
    }
}

impl Solution {
    pub fn minimum_visited_cells(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        let mut tree_col = vec![FenwTreeMinSuff::new(n, i64::MAX); m];
        let mut tree_row = vec![FenwTreeMinSuff::new(m, i64::MAX); n];
        tree_col[0].relax_point(0, 1);
        tree_row[0].relax_point(0, 1);
        for i in 0..n {
            for j in 0..m {
                let dp = tree_col[j].get_min(i).min(tree_row[i].get_min(j));
                if dp == i64::MAX {
                    continue;
                }
                if i + 1 == n && j + 1 == m {
                    return dp as i32;
                }
                let len = g[i][j] as usize;
                tree_col[j].relax_point((i + len).min(n - 1), dp + 1);
                tree_row[i].relax_point((j + len).min(m - 1), dp + 1);
            }
        }
        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![3,4,2,1],vec![4,2,3,1],vec![2,1,0,0],vec![2,4,0,0]], 4)]
    #[case(vec![vec![3,4,2,1],vec![4,2,1,1],vec![2,1,1,0],vec![3,4,1,0]], 3)]
    #[case(vec![vec![2,1,0],vec![1,0,0]], -1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_visited_cells(grid);
        assert_eq!(actual, expected);
    }
}
