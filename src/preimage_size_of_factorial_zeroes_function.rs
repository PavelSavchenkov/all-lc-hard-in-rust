//! Solution for https://leetcode.com/problems/preimage-size-of-factorial-zeroes-function
//! 793. Preimage Size of Factorial Zeroes Function

fn f(mut x: u64) -> u64 {
    let mut ans = 0;
    while x > 0 {
        x /= 5;
        ans += x;
    }
    ans
}

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let k = k as u64;

        if k == 0 {
            return 5;
        }

        let mut bounds = Vec::new();
        for it in 0..2 {
            let mut L = 0;
            let mut R = (k + 1) * 5 + 1;
            while L + 1 != R {
                let M = (L + R) / 2;
                let cond = if it == 0 { f(M) < k } else { f(M) <= k };
                if cond {
                    L = M;
                } else {
                    R = M;
                }
            }
            bounds.push(L);
        }
        (bounds[1] - bounds[0]) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(0, 5)]
    #[case(5, 0)]
    #[case(3, 5)]
    fn case(#[case] k: i32, #[case] expected: i32) {
        let actual = Solution::preimage_size_fzf(k);
        assert_eq!(actual, expected);
    }
}
