//! Solution for https://leetcode.com/problems/substring-with-largest-variance
//! 2272. Substring With Largest Variance

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

const A: usize = 26;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let s = to_u8(&s);

        let mut ans = 0;
        for c1 in 0..A {
            for c2 in 0..A {
                let mut t = Vec::<usize>::new();
                for ch in &s {
                    let c = (ch - b'a') as usize;
                    if c == c1 {
                        t.push(0);
                    } else if c == c2 {
                        t.push(1);
                    }
                }

                let n = t.len();
                let mut pref = vec![vec![0; 2]; n + 1];
                for i in 0..n {
                    for c in 0..2 {
                        pref[i + 1][c] = pref[i][c];
                    }
                    pref[i + 1][t[i]] += 1;
                }

                let mut max_pref = vec![0; n];
                for i in 0..n {
                    let func = -pref[i][0] + pref[i][1];
                    max_pref[i] = func;
                    if i > 0 {
                        max_pref[i] = max_pref[i].max(max_pref[i - 1]);
                    }
                }

                let mut last: Vec<Option<usize>> = vec![None; 2];
                for r in 1..=n {
                    last[t[r - 1]] = Some(r - 1);

                    if last[0].is_none() || last[1].is_none() {
                        continue;
                    }
                    let max_l = last[0].unwrap().min(last[1].unwrap());
                    assert!(max_l < r);
                    let cur = pref[r][0] - pref[r][1] + max_pref[max_l];
                    ans = ans.max(cur);
                }
            }
        }

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aababbb", 3)]
    #[case("abcde", 0)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::largest_variance(s);
        assert_eq!(actual, expected);
    }
}
