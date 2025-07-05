//! Solution for https://leetcode.com/problems/strange-printer-ii
//! 1591. Strange Printer II

#[derive(Clone, Copy, Debug)]
struct Bbox {
    min_i: usize,
    max_i: usize,
    min_j: usize,
    max_j: usize,
}

impl Bbox {
    fn new() -> Self {
        Self {
            min_i: usize::MAX,
            max_i: usize::MIN,
            min_j: usize::MAX,
            max_j: usize::MIN,
        }
    }

    fn add(&mut self, i: usize, j: usize) {
        self.min_i = self.min_i.min(i);
        self.max_i = self.max_i.max(i);
        self.min_j = self.min_j.min(j);
        self.max_j = self.max_j.max(j);
    }
}

impl Solution {
    pub fn is_printable(g: Vec<Vec<i32>>) -> bool {
        let n = g.len();
        let m = g[0].len();

        let mut g: Vec<Vec<usize>> = g
            .iter()
            .map(|row| row.iter().map(|&x| x as usize).collect())
            .collect();
        let colors = g.iter().fold(1, |acc, e| acc.max(*e.iter().max().unwrap()));
        let mut bbox = vec![Bbox::new(); colors + 1];
        for i in 0..n {
            for j in 0..m {
                bbox[g[i][j]].add(i, j);
            }
        }

        loop {
            let mut was_removed = false;
            let mut all_done = true;
            for c in 1..=colors {
                let b = bbox[c];
                let mut can_remove = true;
                let mut was_alive = false;
                for i in b.min_i..=b.max_i {
                    for j in b.min_j..=b.max_j {
                        if g[i][j] == c {
                            all_done = false;
                            was_alive = true;
                        } else if g[i][j] != 0 {
                            can_remove = false;
                            break;
                        }
                    }
                }
                // todo: some kind of handler -- iterate over all i,j in a box with lambda
                if can_remove && was_alive {
                    for i in b.min_i..=b.max_i {
                        for j in b.min_j..=b.max_j {
                            g[i][j] = 0;
                        }
                    }
                    was_removed = true;
                }
            }
            if all_done {
                break;
            }
            if !was_removed {
                return false;
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
    #[case(vec![vec![1,1,1,1],vec![1,2,2,1],vec![1,2,2,1],vec![1,1,1,1]], true)]
    #[case(vec![vec![1,1,1,1],vec![1,1,3,3],vec![1,1,3,4],vec![5,5,1,4]], true)]
    #[case(vec![vec![1,2,1],vec![2,1,2],vec![1,2,1]], false)]
    fn case(#[case] target_grid: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::is_printable(target_grid);
        assert_eq!(actual, expected);
    }
}
