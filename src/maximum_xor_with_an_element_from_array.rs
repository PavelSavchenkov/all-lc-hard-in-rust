//! Solution for https://leetcode.com/problems/maximum-xor-with-an-element-from-array
//! 1707. Maximum XOR With an Element From Array

const B: usize = 32;

type TriePtr = Option<Box<Trie>>;

struct Trie {
    to: Vec<TriePtr>,
    min: i32,
}

impl Trie {
    fn new() -> Self {
        Self {
            to: (0..2).map(|i| None).collect(),
            min: i32::MAX,
        }
    }

    fn add(&mut self, val: i32) {
        let mut t = self;
        t.min = t.min.min(val);
        for i in (0..B).rev() {
            let bit = ((val >> i) & 1) as usize;
            t = t.to[bit].get_or_insert(Box::new(Trie::new()));
            t.min = t.min.min(val);
        }
    }
}

impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut root = Trie::new();
        for &num in &nums {
            root.add(num);
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in &queries {
            let x = q[0];
            let m = q[1];

            if root.min > m {
                ans.push(-1);
                continue;
            }

            let mut max = 0;
            let mut t = &root;
            for i in (0..B).rev() {
                let bit = ((x >> i) & 1) as usize;
                if t.to[bit ^ 1].is_some() && t.to[bit ^ 1].as_ref().unwrap().min <= m {
                    t = t.to[bit ^ 1].as_ref().unwrap();
                    max ^= 1 << i;
                } else {
                    t = t.to[bit].as_ref().unwrap();
                }
            }
            ans.push(max);
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
    #[case(vec![0,1,2,3,4], vec![vec![3,1],vec![1,3],vec![5,6]], vec![3,3,7])]
    #[case(vec![5,2,4,6,6,3], vec![vec![12,4],vec![8,1],vec![6,3]], vec![15,-1,5])]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::maximize_xor(nums, queries);
        assert_eq!(actual, expected);
    }
}
