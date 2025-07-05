//! Solution for https://leetcode.com/problems/frequencies-of-shortest-supersequences
//! 3435. Frequencies of Shortest Supersequences

fn dfs(v: usize, g: &Vec<Vec<usize>>, color: &mut Vec<i32>) -> bool {
    assert!(color[v] == 0);
    color[v] = 1;
    for &to in &g[v] {
        if color[to] == 1 {
            return false;
        }
        if color[to] == 0 {
            if !dfs(to, g, color) {
                return false;
            }
        }
    }
    color[v] = 2;
    true
}

fn has_cycle(g: &Vec<Vec<usize>>) -> bool {
    let n = g.len();
    let mut color = vec![0; n];
    for v in 0..n {
        if color[v] == 0 {
            if !dfs(v, g, &mut color) {
                return true;
            }
        }
    }
    false
}

const A: usize = 26;

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let words = to_u8_vec(&words);

        let mut chars = Vec::new();
        for w in &words {
            for &ch in w {
                chars.push(ch);
            }
        }
        chars.sort();
        chars.dedup();
        assert!(chars.len() <= 16);

        let mut ans_masks = Vec::new();
        let mut len_ans = chars.len() + 1;
        for mask in 0..(1 << chars.len()) as usize {
            let n = chars.len() * 2;
            let mut g = vec![Vec::new(); n];
            let mut lc = vec![0; chars.len()];
            let mut rc = vec![0; chars.len()];
            for c in 0..chars.len() {
                lc[c] = 2 * c;
                if ((mask >> c) & 1) == 1 {
                    rc[c] = 2 * c + 1;
                } else {
                    rc[c] = 2 * c;
                }
            }
            for w in &words {
                let c1 = chars.binary_search(&w[0]).unwrap();
                let c2 = chars.binary_search(&w[1]).unwrap();
                g[lc[c1]].push(rc[c2]);
            }

            if !has_cycle(&g) {
                let len = mask.count_ones() as usize;
                if len < len_ans {
                    ans_masks.clear();
                    len_ans = len;
                }
                if len == len_ans {
                    ans_masks.push(mask);
                }
            }
        }

        let mut freqs = Vec::new();
        for &mask in &ans_masks {
            let mut freq = vec![0; A];
            for c in 0..chars.len() {
                let real_c = (chars[c] - b'a') as usize;
                freq[real_c] += 1;
                if ((mask >> c) & 1) == 1 {
                    freq[real_c] += 1;
                }
            }
            freqs.push(freq);
        }
        freqs
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
    #[case(vec!["ab".into(),"ba".into()], todo!("todo!(\"Failed to get solutions\""))]
    #[case(vec!["aa".into(),"ac".into()], todo!("todo!(\"Failed to get solutions\""))]
    #[case(vec!["aa".into(),"bb".into(),"cc".into()], todo!("todo!(\"Failed to get solutions\""))]
    fn case(#[case] words: Vec<String>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::supersequences(words);
        assert_eq!(actual, expected);
    }
}
