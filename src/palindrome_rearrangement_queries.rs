//! Solution for https://leetcode.com/problems/palindrome-rearrangement-queries
//! 2983. Palindrome Rearrangement Queries

const A: usize = 26;

impl Solution {
    pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let s = to_u8(&s);
        let n = s.len();
        assert!(n % 2 == 0);

        let first_half = s[0..n / 2].to_vec();
        let mut second_half = s[n / 2..n].to_vec();
        second_half.reverse();
        let s = vec![first_half, second_half];
        let n = n / 2;

        let mut cnt = vec![vec![vec![0; A]; n + 1]; 2];
        for it in 0..2 {
            for i in 0..n {
                cnt[it][i + 1] = cnt[it][i].clone();
                let c = (s[it][i] - b'a') as usize;
                cnt[it][i + 1][c] += 1;
            }
        }

        let mut is_bad = vec![0; n];
        let mut pref_bad = vec![0; n + 1];
        for i in 0..n {
            if s[0][i] != s[1][i] {
                is_bad[i] = 1;
            }
            pref_bad[i + 1] = pref_bad[i] + is_bad[i];
        }

        let sub_vec = |a: &Vec<usize>, b: &Vec<usize>| -> Vec<usize> {
            assert!(a.len() == b.len());
            let mut res = vec![0; a.len()];
            for i in 0..a.len() {
                if a[i] < b[i] {
                    return Vec::new();
                }
                res[i] = a[i] - b[i];
            }
            res
        };

        let cnt_seg = |it: usize, l: usize, r: usize| -> Vec<usize> {
            assert!(l < r);
            assert!(r <= n);
            let res = sub_vec(&cnt[it][r], &cnt[it][l]);
            assert!(res.len() == A);
            res
        };

        let cnt_bad = |l: usize, r: usize| -> usize {
            assert!(l < r);
            assert!(r <= n);
            pref_bad[r] - pref_bad[l]
        };

        let mut ans = vec![true; queries.len()];
        for id in 0..queries.len() {
            let q = &queries[id];
            let a = q[0] as usize;
            let b = q[1] as usize + 1;
            let d = n - (q[2] as usize - n);
            let c = n - 1 - (q[3] as usize - n);

            assert!(a < b);
            assert!(c < d);

            let mut es = Vec::new();
            es.push((0, 0));
            es.push((a, 1));
            es.push((b, -1));
            es.push((c, 2));
            es.push((d, -2));
            es.push((n, 0));
            es.sort();

            let mut cnt = vec![cnt_seg(0, a, b), cnt_seg(1, c, d)];
            let mut bal = 0;
            let mut i = 0;
            loop {
                let mut j = i;
                while j < es.len() && es[i].0 == es[j].0 {
                    let code = es[j].1;
                    if code < 0 {
                        bal &= !(-code);
                    } else if code > 0 {
                        bal |= code;
                    }
                    j += 1;
                }
                if j == es.len() {
                    break;
                }
                let l = es[i].0;
                let r = es[j].0;
                assert!(l < r);

                if bal == 0 {
                    if cnt_bad(l, r) != 0 {
                        ans[id] = false;
                        break;
                    }
                } else if bal == 1 {
                    cnt[0] = sub_vec(&cnt[0], &cnt_seg(1, l, r));
                    if cnt[0].is_empty() {
                        ans[id] = false;
                        break;
                    }
                } else if bal == 2 {
                    cnt[1] = sub_vec(&cnt[1], &cnt_seg(0, l, r));
                    if cnt[1].is_empty() {
                        ans[id] = false;
                        break;
                    }
                } else {
                    assert!(bal == 3);
                }

                i = j;
            }

            if cnt[0] != cnt[1] {
                ans[id] = false;
            }
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
    #[case("abcabc", vec![vec![1,1,3,5],vec![0,2,5,5]], vec![true,true])]
    #[case("abbcdecbba", vec![vec![0,2,7,9]], vec![false])]
    #[case("acbcab", vec![vec![1,2,4,5]], vec![true])]
    #[case("stgjtzqwgkuadjgqugkwdtzast", vec![vec![5,10,13,23]], vec![false])]
    fn case(#[case] s: String, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<bool>) {
        let actual = Solution::can_make_palindrome_queries(s, queries);
        assert_eq!(actual, expected);
    }
}
