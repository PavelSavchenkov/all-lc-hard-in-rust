//! Solution for https://leetcode.com/problems/count-special-integers
//! 2376. Count Special Integers

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let mut n = (n + 1) as usize;
        let digits = {
            let mut d = Vec::new();
            while n > 0 {
                d.push(n % 10);
                n /= 10;
            }
            d.reverse();
            d
        };

        let mut binom = vec![vec![0; digits.len() + 1]; 11];
        for from in 0..binom.len() {
            binom[from][0] = 1;
            for k in 1..=from.min(digits.len()) {
                binom[from][k] = binom[from - 1][k] + binom[from - 1][k - 1];
            }
        }

        let mut fact = vec![1; digits.len() + 1];
        for i in 1..fact.len() {
            fact[i] = fact[i - 1] * i;
        }

        let mut ans = 0;
        for len in 1..digits.len() {
            ans += binom[10][len] * fact[len];
            ans -= binom[9][len - 1] * fact[len - 1];
        }

        let mut seen = vec![false; 11];
        for i in 0..digits.len() {
            for cur in 0..digits[i] {
                if cur == 0 && i == 0 {
                    continue;
                }
                if !seen[cur] {
                    let slots = digits.len() - (i + 1);
                    let candidates = 10 - (i + 1);
                    if candidates < slots {
                        continue;
                    }
                    ans += binom[candidates][slots] * fact[slots];
                }
            }
            if seen[digits[i]] {
                break;
            }
            seen[digits[i]] = true;
        }

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
    #[case(20, 19)]
    #[case(5, 5)]
    #[case(135, 110)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::count_special_numbers(n);
        assert_eq!(actual, expected);
    }
}
