//! Solution for https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings
//! 1520. Maximum Number of Non-Overlapping Substrings

struct Seg {
    l: usize,
    r: usize,
}

impl Seg {
    fn len(&self) -> usize {
        self.r - self.l
    }
}

const A: usize = 26;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let s = to_u8(&s);
        let n = s.len();

        let mut min = vec![n; A];
        let mut max = vec![0; A];
        for i in 0..n {
            let c = (s[i] - b'a') as usize;
            if min[c] == n {
                min[c] = i;
            }
            max[c] = i;
        }

        let mut segs = Vec::new();
        for c in 0..A {
            if min[c] > max[c] {
                continue;
            }
            let mut seg = Seg {
                l: min[c],
                r: max[c] + 1,
            };
            let mut used = vec![false; A];
            used[c] = true;
            loop {
                let mut new_c = A;
                for i in seg.l..seg.r {
                    let cc = (s[i] - b'a') as usize;
                    if !used[cc] {
                        new_c = cc;
                        break;
                    }
                }
                if new_c == A {
                    break;
                }
                used[new_c] = true;
                seg.l = seg.l.min(min[new_c]);
                seg.r = seg.r.max(max[new_c] + 1);
            }
            segs.push(seg);
        }

        segs.sort_by_key(|s| s.r);

        let mut dp = vec![(0 as usize, 0); segs.len()];
        let mut prev = vec![segs.len(); segs.len()];
        for i in 0..segs.len() {
            dp[i] = (1 as usize, -(segs[i].len() as i32));
        }
        for i in 0..segs.len() {
            let cur_dp = dp[i];
            for j in i + 1..segs.len() {
                if segs[i].r <= segs[j].l {
                    let mut ndp = cur_dp;
                    ndp.0 += 1;
                    ndp.1 -= segs[j].len() as i32;
                    if ndp > dp[j] {
                        dp[j] = ndp;
                        prev[j] = i;
                    }
                }
            }
        }

        let max = *dp.iter().max().unwrap();
        let mut i = dp.iter().position(|&val| max == val).unwrap();
        let mut ans = Vec::new();
        while i != segs.len() {
            let sub = from_u8(&s[segs[i].l..segs[i].r].to_vec());
            ans.push(sub);
            i = prev[i];
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
    #[case("adefaddaccc", vec!["e".into(),"f".into(),"ccc".into()])]
    #[case("abbaccd", vec!["d".into(),"bb".into(),"cc".into()])]
    fn case(#[case] s: String, #[case] expected: Vec<String>) {
        let actual = Solution::max_num_of_substrings(s);
        assert_eq!(actual, expected);
    }
}
