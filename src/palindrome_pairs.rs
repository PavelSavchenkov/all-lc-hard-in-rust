//! Solution for https://leetcode.com/problems/palindrome-pairs
//! 336. Palindrome Pairs

struct Palindrome {
    odd: Vec<usize>,
    even: Vec<usize>,
    n: usize,
}

impl Palindrome {
    fn new<T: PartialEq>(s: &Vec<T>) -> Self {
        let n = s.len();
        let odd = {
            let mut p = vec![0; n];
            let mut l = 0;
            let mut r = 0;
            for i in 0..n {
                let mut len = 0;
                if r > i {
                    len = p[l + (r - i - 1)].min(r - i - 1);
                }
                while i + len + 1 < n && i >= len + 1 && s[i + len + 1] == s[i - len - 1] {
                    len += 1;
                }
                p[i] = len;
                if i + p[i] + 1 > r {
                    l = i - p[i];
                    r = i + p[i] + 1;
                }
            }
            p
        };
        let even = {
            let mut p = vec![0; n];
            let mut l = 0;
            let mut r = 0;
            for i in 0..n {
                let mut len = 0;
                if r > i {
                    len = p[l + (r - i - 1) + 1].min(r - i);
                }
                while i + len < n && i >= len + 1 && s[i + len] == s[i - len - 1] {
                    len += 1;
                }
                p[i] = len;
                if i + p[i] + 1 > r {
                    l = i - p[i];
                    r = i + p[i];
                }
            }
            p
        };
        Self { odd, even, n }
    }

    fn is_palindrome(&self, l: usize, len: usize) -> bool {
        if len == 0 {
            return true;
        }
        if len % 2 == 0 {
            return self.even[l + len / 2] >= len / 2;
        }
        self.odd[l + len / 2] >= len / 2
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

const A: usize = 26;

type TriePtr = Option<Box<Trie>>;

struct Trie {
    to: Vec<TriePtr>,
    id: Option<usize>,
}

impl Trie {
    fn new() -> Self {
        let mut this = Self {
            to: Vec::new(),
            id: None,
        };
        for c in 0..A {
            this.to.push(None);
        }
        this
    }

    fn add(&mut self, s: &Vec<u8>, id: usize) {
        let mut t = self;
        for &ch in s {
            let c = (ch - b'a') as usize;
            if t.to[c].is_none() {
                t.to[c] = Some(Box::new(Self::new()));
            }
            t = t.to[c].as_mut().unwrap();
        }
        t.id = Some(id);
    }
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut words = to_u8_vec(&words);
        let n = words.len();

        let mut ans = Vec::new();

        for it in 0..2 {
            let mut root = Trie::new();
            for i in 0..n {
                words[i].reverse();
                root.add(&words[i], i);
                words[i].reverse();
            }

            for i in 0..n {
                if words[i].is_empty() {
                    continue;
                }
                let pal = Palindrome::new(&words[i]);
                let mut t = &root;
                let mut up_to = words[i].len();
                if it == 1 {
                    up_to -= 1;
                }
                for j in 0..=up_to {
                    if pal.is_palindrome(j, words[i].len() - j)
                        && t.id.is_some()
                        && i != t.id.unwrap()
                    {
                        let mut pair = vec![i as i32, t.id.unwrap() as i32];
                        if it == 1 {
                            pair.swap(0, 1);
                        }
                        ans.push(pair);
                    }
                    if j == up_to {
                        break;
                    }
                    let c = (words[i][j] - b'a') as usize;
                    if t.to[c].is_none() {
                        break;
                    }
                    t = t.to[c].as_ref().unwrap();
                }
            }

            for i in 0..n {
                words[i].reverse();
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
    #[case(vec!["abcd".into(),"dcba".into(),"lls".into(),"s".into(),"sssll".into()], vec![vec![0,1],vec![1,0],vec![3,2],vec![2,4]])]
    #[case(vec!["bat".into(),"tab".into(),"cat".into()], vec![vec![0,1],vec![1,0]])]
    #[case(vec!["a".into(),"".into()], vec![vec![0,1],vec![1,0]])]
    fn case(#[case] words: Vec<String>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::palindrome_pairs(words);
        assert_eq!(actual, expected);
    }
}
