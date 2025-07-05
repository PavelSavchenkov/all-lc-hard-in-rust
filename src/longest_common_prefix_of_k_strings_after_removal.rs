//! Solution for https://leetcode.com/problems/longest-common-prefix-of-k-strings-after-removal
//! 3485. Longest Common Prefix of K Strings After Removal

const A: usize = 26;

type TriePtr = Option<Box<Trie>>;

#[derive(Clone)]
struct Trie {
    to: Vec<TriePtr>,
    cnt_terminal_sub_tree: usize,
    ans_sub_tree: usize,
    ans: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            to: vec![None; A],
            cnt_terminal_sub_tree: 0,
            ans_sub_tree: 0,
            ans: 0,
        }
    }

    fn add(&mut self, s: &Vec<u8>) {
        let mut t = self;
        t.cnt_terminal_sub_tree += 1;
        for &ch in s {
            let c = (ch - b'a') as usize;
            t = t.to[c].get_or_insert(Box::new(Trie::new()));
            t.cnt_terminal_sub_tree += 1;
        }
    }
}

fn dfs_init(t: &mut Trie, depth: usize, k: usize) {
    let mut mx = 0;
    for c in 0..A {
        if let Some(son) = &mut t.to[c] {
            dfs_init(son, depth + 1, k);
            mx = mx.max(son.ans_sub_tree);
        }
    }
    if t.cnt_terminal_sub_tree >= k {
        mx = mx.max(depth);
    }
    t.ans_sub_tree = mx;
}

fn dfs_ans(t: &mut Trie, depth: usize, mut ans_up: usize, k: usize) {
    if t.cnt_terminal_sub_tree >= k + 1 {
        ans_up = ans_up.max(depth);
    }
    let mut mx1 = 0;
    let mut mx2 = 0;
    for c in 0..A {
        if let Some(son) = &t.to[c] {
            let len = son.ans_sub_tree;
            if len > mx1 {
                mx2 = mx1;
                mx1 = len;
            } else if len > mx2 {
                mx2 = len;
            }
        }
    }
    t.ans = ans_up.max(mx1);
    for c in 0..A {
        if let Some(son) = &mut t.to[c] {
            let len = son.ans_sub_tree;
            let mut cur_ans_up = ans_up;
            if len == mx1 {
                cur_ans_up = cur_ans_up.max(mx2);
            } else {
                cur_ans_up = cur_ans_up.max(mx1);
            }
            dfs_ans(son, depth + 1, cur_ans_up, k);
        }
    }
}

impl Solution {
    pub fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
        let words = to_u8_vec(&words);
        let k = k as usize;

        let mut trie = Trie::new();
        for w in &words {
            trie.add(w);
        }

        dfs_init(&mut trie, 0, k);
        dfs_ans(&mut trie, 0, 0, k);

        let mut ans = Vec::new();
        for w in &words {
            let mut t = &trie;
            for &ch in w {
                let c = (ch - b'a') as usize;
                t = t.to[c].as_ref().unwrap();
            }
            ans.push(t.ans);
        }
        ans.iter().map(|&x| x as i32).collect()
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
    #[case(vec!["jump".into(),"run".into(),"run".into(),"jump".into(),"run".into()], 2, vec![3,4,4,3,4])]
    #[case(vec!["dog".into(),"racer".into(),"car".into()], 2, vec![0,0,0])]
    fn case(#[case] words: Vec<String>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::longest_common_prefix(words, k);
        assert_eq!(actual, expected);
    }
}
