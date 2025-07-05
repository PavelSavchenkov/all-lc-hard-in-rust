//! Solution for https://leetcode.com/problems/permutation-sequence
//! 60. Permutation Sequence

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize;

        let mut fact = vec![1; n];
        for i in 1..fact.len() {
            fact[i] = fact[i - 1] * i;
        }

        let mut ans = Vec::new();
        let mut used = vec![false; n];
        for i in 0..n {
            for f in 0..n {
                if used[f] {
                    continue;
                }
                let to_skip = fact[n - i - 1];
                if k <= to_skip {
                    ans.push(std::char::from_digit((f + 1) as u32, 10).unwrap());
                    used[f] = true;
                    break;
                }
                k -= to_skip;
            }
            assert!(ans.len() == i + 1);
        }

        ans.iter().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 3, "213")]
    #[case(4, 9, "2314")]
    #[case(3, 1, "123")]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::get_permutation(n, k);
        assert_eq!(actual, expected);
    }
}
