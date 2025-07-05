//! Solution for https://leetcode.com/problems/subsequence-with-the-minimum-score
//! 2565. Subsequence With the Minimum Score

// pref[i] -- how many characters from prefix of s we need to have t[0..i) as a subseq
fn precalc_pref(s: &Vec<u8>, t: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let m = t.len();
    let mut cnt = vec![n + 1; m + 1];
    cnt[0] = 0;
    for i in 0..m {
        let mut j = cnt[i];
        while j < n && s[j] != t[i] {
            j += 1
        }
        if j == n {
            break;
        }
        cnt[i + 1] = j + 1;
    }
    cnt
}

impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
        let mut s = to_u8(&s);
        let mut t = to_u8(&t);

        let pref = precalc_pref(&s, &t);

        s.reverse();
        t.reverse();
        let suff = precalc_pref(&s, &t);
        s.reverse();
        t.reverse();

        let mut ans = t.len();
        let mut len_suff = t.len();
        for len_pref in 0..=t.len() {
            while len_suff > 0 && pref[len_pref] + suff[len_suff] > s.len() {
                len_suff -= 1;
            }
            if pref[len_pref] + suff[len_suff] > s.len() {
                break;
            }
            ans = ans.min(t.len() - len_pref - len_suff);
        }

        ans as i32
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abacaba", "bzaa", 1)]
    #[case("cde", "xyz", 3)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: i32) {
        let actual = Solution::minimum_score(s, t);
        assert_eq!(actual, expected);
    }
}
