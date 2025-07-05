//! Solution for https://leetcode.com/problems/n-queens
//! 51. N-Queens

const N: usize = 9;

type Grid = Vec<Vec<u8>>;

fn grid_to_str(g: &Grid) -> Vec<String> {
    let mut ans = Vec::new();
    for row in g {
        ans.push(String::from_utf8(row.to_vec()).unwrap());
    }
    ans
}

#[derive(Default)]
struct State {
    cols: [bool; N],
    sum: [bool; N + N],
    sub: [bool; N + N],
    grid: Grid,
}

impl State {
    fn new(n: usize) -> Self {
        let mut this = Self::default();
        this.grid = vec![vec![b'.'; n]; n];
        this
    }
}

fn go(s: &mut State, row: usize, n: usize, ans: &mut Vec<Grid>) {
    if row == n {
        ans.push(s.grid.clone());
        return;
    }

    for col in 0..n {
        if s.cols[col] {
            continue;
        }
        let sum = row + col;
        if s.sum[sum] {
            continue;
        }
        let sub = n + row - col;
        if s.sub[sub] {
            continue;
        }

        s.cols[col] = true;
        s.sum[sum] = true;
        s.sub[sub] = true;

        s.grid[row][col] = b'Q';
        go(s, row + 1, n, ans);
        s.grid[row][col] = b'.';

        s.cols[col] = false;
        s.sum[sum] = false;
        s.sub[sub] = false;
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut ans = Vec::<Grid>::new();
        let mut s = State::new(n);
        go(&mut s, 0, n, &mut ans);

        ans.iter().map(grid_to_str).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![".Q..".into(),"...Q".into(),"Q...".into(),"..Q.".into()],vec!["..Q.".into(),"Q...".into(),"...Q".into(),".Q..".into()]])]
    #[case(1, vec![vec!["Q".into()]])]
    fn case(#[case] n: i32, #[case] expected: Vec<Vec<String>>) {
        let actual = Solution::solve_n_queens(n);
        assert_eq!(actual, expected);
    }
}
