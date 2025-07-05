//! Solution for https://leetcode.com/problems/maximum-running-time-of-n-computers
//! 2141. Maximum Running Time of N Computers

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let n = n as usize;

        let can = |T: i64| -> bool {
            let mut sum = 0;
            for &b in &batteries {
                sum += T.min(b as i64);
            }
            sum >= n as i64 * T
        };

        let mut L: i64 = 0;
        let mut R: i64 = batteries.iter().map(|&b| b as i64).sum::<i64>() / n as i64 + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                L = M
            } else {
                R = M
            }
        }

        L
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![3,3,3], 4)]
    #[case(2, vec![1,1,1,1], 2)]
    fn case(#[case] n: i32, #[case] batteries: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::max_run_time(n, batteries);
        assert_eq!(actual, expected);
    }
}
