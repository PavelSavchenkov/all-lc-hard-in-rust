//! Solution for https://leetcode.com/problems/maximize-value-of-function-in-a-ball-passing-game
//! 2836. Maximize Value of Function in a Ball Passing Game

impl Solution {
    pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
        let k = k as u64;
        let n = receiver.len();

        let mut log = 0;
        while (1 << log) < k {
            log += 1;
        }
        log += 1;

        let mut sum = vec![vec![0; log]; n];
        let mut next = vec![vec![0; log]; n];
        for i in 0..n {
            next[i][0] = receiver[i] as usize;
            sum[i][0] = receiver[i] as u64;
        }
        for l in 1..log {
            for i in 0..n {
                next[i][l] = next[next[i][l - 1]][l - 1];
                sum[i][l] = sum[i][l - 1] + sum[next[i][l - 1]][l - 1];
            }
        }

        let mut ans = 0;
        for i in 0..n {
            let mut passes = k;
            let mut j = i;
            let mut cur = i as u64;
            for l in (0..log).rev() {
                if (1 << l) <= passes {
                    cur += sum[j][l];
                    passes -= 1 << l;
                    j = next[j][l];
                }
            }
            assert!(passes == 0);
            ans = ans.max(cur);
        }
        ans as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,0,1], 4, 6)]
    #[case(vec![1,1,1,2,3], 3, 10)]
    fn case(#[case] receiver: Vec<i32>, #[case] k: i64, #[case] expected: i64) {
        let actual = Solution::get_max_function_value(receiver, k);
        assert_eq!(actual, expected);
    }
}
