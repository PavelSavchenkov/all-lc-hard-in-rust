//! Solution for https://leetcode.com/problems/shortest-matching-substring
//! 3455. Shortest Matching Substring

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

fn calc_next_occurences(s: &Vec<u8>, p: &Vec<u8>) -> Vec<usize> {
    if p.is_empty() {
        return (0..=s.len()).collect();
    }
    let mut ss = p.clone();
    ss.extend(s.iter());

    let z = calc_z(&ss);

    let mut next_occur = vec![s.len() + 1; s.len() + 1];
    for i in (0..s.len()).rev() {
        next_occur[i] = next_occur[i + 1];
        if z[p.len() + i] >= p.len() {
            next_occur[i] = i;
        }
    }
    next_occur
}

impl Solution {
    pub fn shortest_matching_substring(s: String, p: String) -> i32 {
        let s = to_u8(&s);
        let p = to_u8(&p);

        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        let mut cnt_star = 0;
        for &ch in &p {
            if ch == b'*' {
                cnt_star += 1;
            } else {
                match cnt_star {
                    0 => {
                        a.push(ch);
                    }
                    1 => {
                        b.push(ch);
                    }
                    2 => {
                        c.push(ch);
                    }
                    _ => panic!(),
                }
            }
        }

        let next_a = calc_next_occurences(&s, &a);
        let next_b = calc_next_occurences(&s, &b);
        let next_c = calc_next_occurences(&s, &c);

        let mut min_len = usize::MAX;
        for i in 0..s.len() {
            if next_a[i] != i {
                continue;
            }
            let mut j = i + a.len();
            j = next_b[j];
            if j > s.len() {
                continue;
            }
            j += b.len();
            j = next_c[j];
            if j > s.len() {
                continue;
            }
            j += c.len();
            let len = j - i;
            if len < min_len {
                min_len = len;
            }
        }
        if min_len == usize::MAX {
            return -1;
        }
        min_len as i32
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
    #[case("abaacbaecebce", "ba*c*ce", 8)]
    #[case("baccbaadbc", "cc*baa*adb", -1)]
    #[case("a", "**", 0)]
    #[case("madlogic", "*adlogi*", 6)]
    fn case(#[case] s: String, #[case] p: String, #[case] expected: i32) {
        let actual = Solution::shortest_matching_substring(s, p);
        assert_eq!(actual, expected);
    }
}
