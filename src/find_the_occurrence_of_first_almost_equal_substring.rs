//! Solution for https://leetcode.com/problems/find-the-occurrence-of-first-almost-equal-substring
//! 3303. Find the Occurrence of First Almost Equal Substring

fn calc_z(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut l = 0;
    let mut r = 0;
    let mut z = vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[i + z[i]] == s[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}

fn calc_z_occurences(s: &Vec<u8>, pattern: &Vec<u8>) -> Vec<usize> {
    let mut ss = pattern.clone();
    ss.push(0);
    ss.extend(s.iter());
    let z = calc_z(&ss);
    z[pattern.len() + 1..].to_vec()
}

impl Solution {
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        let mut s = to_u8(&s);
        let mut pattern = to_u8(&pattern);

        let pref = calc_z_occurences(&s, &pattern);
        assert!(pref.len() == s.len());

        s.reverse();
        pattern.reverse();
        let mut suff = calc_z_occurences(&s, &pattern);
        assert!(suff.len() == s.len());
        s.reverse();
        pattern.reverse();
        suff.reverse();

        // eprintln!("pref = {:#?}", pref);
        // eprintln!("suff = {:#?}", suff);

        for i in 0..=s.len() - pattern.len() {
            let pref_len = pref[i];
            let suff_len = suff[i + pattern.len() - 1];
            if pref_len + suff_len + 1 >= pattern.len() {
                return i as i32;
            }
        }
        -1
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
    #[case("abcdefg", "bcdffg", 1)]
    #[case("ababbababa", "bacaba", 4)]
    #[case("abcd", "dba", -1)]
    #[case("dde", "d", 0)]
    fn case(#[case] s: String, #[case] pattern: String, #[case] expected: i32) {
        let actual = Solution::min_starting_index(s, pattern);
        assert_eq!(actual, expected);
    }
}
