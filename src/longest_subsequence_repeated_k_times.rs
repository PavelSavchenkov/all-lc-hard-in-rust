//! Solution for https://leetcode.com/problems/longest-subsequence-repeated-k-times
//! 2014. Longest Subsequence Repeated k Times

const A: usize = 26;

fn check(s: &Vec<usize>, k: usize, t: &Vec<usize>) -> bool {
    let mut j = 0;
    for i in 0..k * t.len() {
        while j < s.len() && s[j] != t[i % t.len()] {
            j += 1;
        }
        if j == s.len() {
            return false;
        }
        j += 1;
    }
    true
}

fn go(s: &Vec<usize>, k: usize, cnt: &mut [usize], t: &mut Vec<usize>, ans: &mut Vec<usize>) {
    if check(s, k, t) {
        if t.len() > ans.len() || (t.len() == ans.len() && t > ans) {
            *ans = t.clone();
        }
    } else {
        return;
    }

    for c in 0..A {
        if cnt[c] < k {
            continue;
        }
        cnt[c] -= k;
        t.push(c);
        go(s, k, cnt, t, ans);
        t.pop();
        cnt[c] += k;
    }
}

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let s: Vec<usize> = s.as_bytes().iter().map(|c| (c - b'a') as usize).collect();
        let k = k as usize;

        let mut cnt = [0 as usize; A];
        for &c in &s {
            cnt[c] += 1;
        }

        let mut ans = Vec::<usize>::new();
        let mut t = Vec::<usize>::new();

        go(&s, k, &mut cnt, &mut t, &mut ans);

        String::from_utf8(ans.iter().map(|&c| b'a' + (c as u8)).collect()).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("letsleetcode", 2, "let")]
    #[case("bb", 2, "b")]
    #[case("ab", 2, "")]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::longest_subsequence_repeated_k(s, k);
        assert_eq!(actual, expected);
    }
}
