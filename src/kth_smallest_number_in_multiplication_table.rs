//! Solution for https://leetcode.com/problems/kth-smallest-number-in-multiplication-table
//! 668. Kth Smallest Number in Multiplication Table

fn cnt_less_or_eq(n: u32, m: u32, s: u32) -> u32 {
    let mut ans = 0;
    for i in 1..=n {
        // i * j <= s --> j <= s / i
        ans += m.min(s / i);
    }
    ans
}

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let m = m as u32;
        let n = n as u32;
        let k = k as u32;

        let mut L = 0;
        let mut R = n * m + 1;
        while L + 1 != R {
            let M = (L + R) / 2;

            if cnt_less_or_eq(n, m, M) >= k {
                R = M
            } else {
                L = M
            }
        }
        R as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 3, 5, 3)]
    #[case(2, 3, 6, 6)]
    fn case(#[case] m: i32, #[case] n: i32, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::find_kth_number(m, n, k);
        assert_eq!(actual, expected);
    }
}
