//! Solution for https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii
//! 3445. Maximum Difference Between Even and Odd Frequency II

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let s = to_u8(&s);
        let n = s.len();
        let k = k as usize;

        let mut chars = s.clone();
        chars.sort();
        chars.dedup();
        let A = chars.len();
        let s: Vec<usize> = s.iter().map(|x| chars.binary_search(&x).unwrap()).collect();

        let mut pref = vec![vec![0; n + 1]; A];
        for i in 0..n {
            for c in 0..A {
                pref[c][i + 1] = pref[c][i];
            }
            pref[s[i]][i + 1] = pref[s[i]][i] + 1;
        }
        let mut min_diff = vec![vec![vec![vec![vec![i32::MAX; 2]; 2]; A]; A]; n + 2];
        for L in 0..=n {
            if L > 0 {
                min_diff[L + 1] = min_diff[L].clone();
            }
            for a in 0..A {
                for b in 0..A {
                    let pa = pref[a][L] % 2;
                    let pb = pref[b][L] % 2;
                    let diff = pref[a][L] as i32 - pref[b][L] as i32;
                    remin(&mut min_diff[L + 1][a][b][pa][pb], diff);
                }
            }
        }

        let mut ans = i32::MIN;
        let mut last_occur = vec![n; A];
        for i in 0..n {
            if i + 1 >= k {
                for a in 0..A {
                    for b in 0..A {
                        if a == b {
                            continue;
                        }
                        if last_occur[b] == n {
                            continue;
                        }
                        let mut l = i + 1 - k;
                        l = l.min(last_occur[b]);
                        let fa = pref[a][i + 1];
                        let fb = pref[b][i + 1];
                        let pa = fa % 2;
                        let pb = fb % 2;
                        let diff = fa as i32 - fb as i32;
                        let sub = min_diff[l + 1][a][b][pa ^ 1][pb];
                        if sub < i32::MAX {
                            // eprintln!(
                            //     "i={}, a={}, b={}, diff={}, sub={}",
                            //     i, chars[a] as char, chars[b] as char, diff, sub
                            // );
                            let cur = diff - sub;
                            ans = ans.max(cur);
                        }
                    }
                }
            }
            last_occur[s[i]] = i;
        }

        ans
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
    #[case("12233", 4, -1)]
    #[case("1122211", 3, 1)]
    #[case("110", 3, -1)]
    #[case("111320214", 9, -1)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_difference(s, k);
        assert_eq!(actual, expected);
    }
}
