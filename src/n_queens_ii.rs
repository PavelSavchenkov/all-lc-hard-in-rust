//! Solution for https://leetcode.com/problems/n-queens-ii
//! 52. N-Queens II

const N: usize = 9;

#[derive(Default)]
struct State {
    cols: [bool; N],
    sum: [bool; N + N],
    sub: [bool; N + N],
}

impl State {
    fn new(n: usize) -> Self {
        Self::default()
    }
}

fn go(s: &mut State, row: usize, n: usize, ans: &mut usize) {
    if row == n {
        *ans += 1;
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

        go(s, row + 1, n, ans);

        s.cols[col] = false;
        s.sum[sum] = false;
        s.sub[sub] = false;
    }
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;

        let mut ans = 0;
        let mut s = State::new(n);
        go(&mut s, 0, n, &mut ans);
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
    #[case(4, 2)]
    #[case(1, 1)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::total_n_queens(n);
        assert_eq!(actual, expected);
    }
}
