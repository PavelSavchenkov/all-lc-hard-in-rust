//! Solution for https://leetcode.com/problems/longest-valid-parentheses
//! 32. Longest Valid Parentheses

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let s = s.as_bytes();
        let n = s.len();
        let mut pair = vec![-1 as i32; n];

        let mut st = Vec::<usize>::new();
        for i in 0..n {
            if !st.is_empty() && s[*st.last().unwrap()] == b'(' && s[i] == b')' {
                pair[i] = *st.last().unwrap() as i32;
                st.pop();
            } else {
                st.push(i);
            }
        }

        let mut dp = vec![0 as usize; n];
        for i in 0..n {
            let j = pair[i];
            if j == -1 {
                continue;
            }
            let j = j as usize;
            let mut ans = i - j + 1;
            if j >= 1 {
                ans += dp[j - 1];
            }
            dp[i] = ans;
        }

        *dp.iter().max().unwrap() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(()", 2)]
    #[case(")()())", 4)]
    #[case("", 0)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::longest_valid_parentheses(s);
        assert_eq!(actual, expected);
    }
}
