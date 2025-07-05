//! Solution for https://leetcode.com/problems/maximum-strong-pair-xor-ii
//! 2935. Maximum Strong Pair XOR II

const B: usize = 22;

type TriePtr = Option<Box<Trie>>;

#[derive(Clone)]
struct Trie {
    to: Vec<TriePtr>,
    size: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            to: vec![None; 2],
            size: 0,
        }
    }

    fn add(&mut self, num: i32) {
        let mut t = self;
        t.size += 1;
        for i in (0..B).rev() {
            let b = ((num >> i) & 1) as usize;
            t = t.to[b].get_or_insert(Box::new(Trie::new()));
            t.size += 1;
        }
    }

    fn remove(&mut self, num: i32) {
        let mut t = self;
        t.size -= 1;
        for i in (0..B).rev() {
            let b = ((num >> i) & 1) as usize;
            t = t.to[b].as_mut().unwrap();
            t.size -= 1;
        }
    }

    fn max_xor(&self, num: i32) -> i32 {
        let mut t = self;
        let mut ans = 0;
        for i in (0..B).rev() {
            let b = ((num >> i) & 1) as usize;
            let mut go = b;
            if let Some(son) = &t.to[b ^ 1] {
                if son.size > 0 {
                    go = b ^ 1;
                }
            }
            t = &t.to[go].as_ref().unwrap();
            if go != b {
                ans ^= 1 << i;
            }
        }
        ans
    }
}

impl Solution {
    pub fn maximum_strong_pair_xor(mut a: Vec<i32>) -> i32 {
        a.sort();
        a.dedup();
        let n = a.len();

        let mut ans = 0;
        let mut trie = Trie::new();
        let mut ptr = 0;
        for i in 0..n {
            if i > 0 {
                trie.remove(a[i - 1]);
            }
            while ptr < n && a[ptr] <= 2 * a[i] {
                trie.add(a[ptr]);
                ptr += 1;
            }
            ans = ans.max(trie.max_xor(a[i]));
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
    #[case(vec![1,2,3,4,5], 7)]
    #[case(vec![10,100], 0)]
    #[case(vec![500,520,2500,3000], 1020)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximum_strong_pair_xor(nums);
        assert_eq!(actual, expected);
    }
}
