//! Solution for https://leetcode.com/problems/lexicographically-smallest-generated-string
//! 3474. Lexicographically Smallest Generated String

fn calc_prefix_function(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            p[i] = j + 1;
        }
    }
    p
}

const A: usize = 26;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let str1 = to_u8(&str1);
        let n = str1.len();
        let str2 = to_u8(&str2);
        let m = str2.len();

        let mut s = vec![0; n + m - 1];
        for i in 0..n {
            if str1[i] == b'T' {
                for j in 0..m {
                    let put = str2[j];
                    if s[i + j] != 0 && s[i + j] != put {
                        return "".to_string();
                    }
                    s[i + j] = put;
                }
            }
        }
        for i in 0..n {
            if str1[i] == b'F' {
                let mut ok = false;
                for j in 0..m {
                    if s[i + j] != str2[j] {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    return "".to_string();
                }
            }
        }

        let mut pref0 = vec![0; s.len() + 1];
        for i in 0..s.len() {
            pref0[i + 1] = pref0[i] + (if s[i] == 0 { 1 } else { 0 });
        }

        let cnt0 = |l: usize, r: usize| -> usize { pref0[r + 1] - pref0[l] };

        let p_func = calc_prefix_function(&str2);
        let mut go = vec![vec![0; A]; str2.len() + 1];
        for p in 0..str2.len() {
            for c in 0..A {
                let ch = b'a' + c as u8;
                if str2[p] == ch {
                    go[p][c] = p + 1;
                } else if p == 0 {
                    go[p][c] = 0;
                } else {
                    let link = p_func[p - 1];
                    assert!(link < p);
                    let rec = go[link][c];
                    go[p][c] = rec;
                }
            }
        }
        for c in 0..A {
            go[str2.len()][c] = go[p_func[str2.len() - 1]][c];
        }

        let mut dp = vec![vec![false; str2.len() + 1]; n + m];
        for p in 0..=str2.len() {
            dp[n + m - 1][p] = true;
        }
        for i in (0..n + m - 1).rev() {
            for p in 0..=str2.len() {
                if p > i {
                    break;
                }
                let mut cur_dp = false;
                for c in 0..A {
                    if s[i] != 0 && s[i] != (b'a' + c as u8) {
                        continue;
                    }
                    if go[p][c] == str2.len() && cnt0(i + 1 - p, i) != 0 {
                        continue;
                    }
                    let np = go[p][c];
                    cur_dp |= dp[i + 1][np];
                }
                dp[i][p] = cur_dp;
            }
        }

        let mut p = 0;
        for i in 0..s.len() {
            if s[i] == 0 {
                for c in 0..A {
                    let np = go[p][c];
                    if np == str2.len() && cnt0(i + 1 - np, i) != 0 {
                        continue;
                    }
                    if dp[i + 1][np] {
                        let ch = b'a' + c as u8;
                        s[i] = ch;
                        break;
                    }
                }
            }
            if s[i] == 0 {
                return "".to_string();
            }
            p = go[p][(s[i] - b'a') as usize];
            if p == str2.len() {
                assert!(cnt0(i + 1 - p, i) == 0);
            }
        }

        from_u8(&s)
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
    #[case("TFTF", "ab", "ababa")]
    #[case("TFTF", "abc", "")]
    #[case("F", "d", "a")]
    fn case(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let actual = Solution::generate_string(str1, str2);
        assert_eq!(actual, expected);
    }
}
