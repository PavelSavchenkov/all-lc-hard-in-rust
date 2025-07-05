//! Solution for https://leetcode.com/problems/grid-illumination
//! 1001. Grid Illumination

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Field {
    cnt_col: HashMap<i32, usize>,
    cnt_row: HashMap<i32, usize>,
    cnt_diag1: HashMap<i32, usize>,
    cnt_diag2: HashMap<i32, usize>,
    turned_on: HashSet<(i32, i32)>,
}

impl Field {
    fn new() -> Self {
        Self::default()
    }

    fn turn_on(&mut self, row: i32, col: i32) {
        if self.turned_on.contains(&(row, col)) {
            return;
        }
        self.turned_on.insert((row, col));
        *self.cnt_col.entry(col).or_insert(0) += 1;
        *self.cnt_row.entry(row).or_insert(0) += 1;
        *self.cnt_diag1.entry(row - col).or_insert(0) += 1;
        *self.cnt_diag2.entry(row + col).or_insert(0) += 1;
    }

    fn turn_off(&mut self, row: i32, col: i32) {
        if self.turned_on.remove(&(row, col)) {
            *self.cnt_col.get_mut(&col).unwrap() -= 1;
            *self.cnt_row.get_mut(&row).unwrap() -= 1;
            *self.cnt_diag1.get_mut(&(row - col)).unwrap() -= 1;
            *self.cnt_diag2.get_mut(&(row + col)).unwrap() -= 1;
        }
    }

    fn is_illuminated(&self, row: i32, col: i32) -> bool {
        if *self.cnt_col.get(&col).unwrap_or(&0) > 0 {
            return true;
        }
        if *self.cnt_row.get(&row).unwrap_or(&0) > 0 {
            return true;
        }
        if *self.cnt_diag1.get(&(row - col)).unwrap_or(&0) > 0 {
            return true;
        }
        if *self.cnt_diag2.get(&(row + col)).unwrap_or(&0) > 0 {
            return true;
        }
        false
    }
}

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut field = Field::new();
        for lamp in &lamps {
            let row = lamp[0];
            let col = lamp[1];
            field.turn_on(row, col);
        }

        let mut ans = Vec::new();
        for q in &queries {
            let row = q[0];
            let col = q[1];
            if field.is_illuminated(row, col) {
                ans.push(1);
            } else {
                ans.push(0);
            }
            for r in row - 1..=row + 1 {
                for c in col - 1..=col + 1 {
                    field.turn_off(r, c);
                }
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
    #[case(5, vec![vec![0,0],vec![4,4]], vec![vec![1,1],vec![1,0]], vec![1,0])]
    #[case(5, vec![vec![0,0],vec![4,4]], vec![vec![1,1],vec![1,1]], vec![1,1])]
    #[case(5, vec![vec![0,0],vec![0,4]], vec![vec![0,4],vec![0,1],vec![1,4]], vec![1,1,0])]
    fn case(
        #[case] n: i32,
        #[case] lamps: Vec<Vec<i32>>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::grid_illumination(n, lamps, queries);
        assert_eq!(actual, expected);
    }
}
