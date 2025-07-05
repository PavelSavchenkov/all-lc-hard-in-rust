//! Solution for https://leetcode.com/problems/stamping-the-grid
//! 2132. Stamping the Grid

#[derive(Clone)]
struct SumGrid {
    s: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    did_precalc: bool,
}

impl SumGrid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            did_precalc: false,
            s: vec![vec![0; cols + 1]; rows + 1],
        }
    }

    // [r0, r1), [c0, c1)
    fn add_on_rect(&mut self, r0: usize, r1: usize, c0: usize, c1: usize, val: i32) {
        assert!(!self.did_precalc);
        self.s[r0][c0] += val;
        self.s[r1][c0] -= val;
        self.s[r0][c1] -= val;
        self.s[r1][c1] += val;
    }

    fn precalc(&mut self) {
        assert!(!self.did_precalc);
        let mut pt = vec![vec![0; self.cols + 1]; self.rows + 1];
        for r in (0..self.rows).rev() {
            for c in (0..self.cols).rev() {
                pt[r][c] = self.s[r + 1][c + 1];
                pt[r][c] += pt[r + 1][c];
                pt[r][c] += pt[r][c + 1];
                pt[r][c] -= pt[r + 1][c + 1];
            }
        }

        let mut s = vec![vec![0; self.cols + 1]; self.rows + 1];
        for r in 0..self.rows {
            for c in 0..self.cols {
                s[r + 1][c + 1] = pt[r][c] + s[r][c + 1] + s[r + 1][c] - s[r][c];
            }
        }
        self.did_precalc = true;
        self.s = s;
    }

    fn get_sum(&self, r0: usize, r1: usize, c0: usize, c1: usize) -> i32 {
        assert!(self.did_precalc);
        let mut sum = 0;
        sum += self.s[r1][c1];
        sum -= self.s[r1][c0];
        sum -= self.s[r0][c1];
        sum += self.s[r0][c0];
        sum
    }
}

impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let h = stamp_height as usize;
        let w = stamp_width as usize;

        let rows = grid.len();
        let cols = grid[0].len();

        let mut init_grid = SumGrid::new(rows, cols);

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    init_grid.add_on_rect(r, r + 1, c, c + 1, 1);
                }
            }
        }

        let mut cover_grid = init_grid.clone();
        init_grid.precalc();

        for r in 0..rows {
            if r + h > rows {
                break;
            }
            for c in 0..cols {
                if c + w > cols {
                    break;
                }
                if init_grid.get_sum(r, r + h, c, c + w) == 0 {
                    cover_grid.add_on_rect(r, r + h, c, c + w, 1);
                    // eprintln!("Add on rect: r={}, c={}, h={}, w={}", r, c, h, w);
                }
            }
        }
        cover_grid.precalc();

        for r in 0..rows {
            for c in 0..cols {
                if cover_grid.get_sum(r, r + 1, c, c + 1) == 0 {
                    return false;
                }
            }
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,0,0,0],vec![1,0,0,0],vec![1,0,0,0],vec![1,0,0,0],vec![1,0,0,0]], 4, 3, true)]
    #[case(vec![vec![1,0,0,0],vec![0,1,0,0],vec![0,0,1,0],vec![0,0,0,1]], 2, 2, false )]
    fn case(
        #[case] grid: Vec<Vec<i32>>,
        #[case] stamp_height: i32,
        #[case] stamp_width: i32,
        #[case] expected: bool,
    ) {
        let actual = Solution::possible_to_stamp(grid, stamp_height, stamp_width);
        assert_eq!(actual, expected);
    }
}
