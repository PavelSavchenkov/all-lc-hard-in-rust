//! Solution for https://leetcode.com/problems/consecutive-numbers-sum
//! 829. Consecutive Numbers Sum

fn sum(l: u64, cnt: u64) -> u64 {
    assert!(cnt > 0);
    (l + l + cnt - 1) * cnt / 2
}

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let n = n as u64;

        let mut ans = 0;
        let mut cnt = 1;
        while sum(1, cnt) <= n {
            let mut L = 1;
            let mut R = n + 1;
            while L + 1 != R {
                let M = (L + R) / 2;
                if sum(M, cnt) <= n {
                    L = M;
                } else {
                    R = M;
                }
            }
            if sum(L, cnt) == n {
                ans += 1;
            }
            cnt += 1;
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
    #[case(5, 2)]
    #[case(9, 3)]
    #[case(15, 4)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::consecutive_numbers_sum(n);
        assert_eq!(actual, expected);
    }
}
