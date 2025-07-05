//! Solution for https://leetcode.com/problems/maximum-genetic-difference-query
//! 1938. Maximum Genetic Difference Query

struct Trie {
    to: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Self { to: [None, None] }
    }

    fn add(&mut self, num: u32, bit: i32) {
        assert!(bit >= 0);
        let b = ((num >> bit) & 1) as usize;
        if self.to[b].is_none() {
            self.to[b] = Some(Box::new(Trie::new()));
        }
        if bit > 0 {
            self.to[b].as_mut().unwrap().add(num, bit - 1);
        }
    }

    fn rem(&mut self, num: u32, bit: i32) -> bool {
        if bit == -1 {
            return false;
        }
        let b = ((num >> bit) & 1) as usize;
        assert!(!self.to[b].is_none());
        let child = self.to[b].as_mut().unwrap();
        if !child.rem(num, bit - 1) {
            self.to[b] = None;
        }
        for bb in 0..2 {
            if !self.to[bb].is_none() {
                return true;
            }
        }
        false
    }
}

const B: i32 = 20;

fn traverse_trie(mut t: &Trie, val: i32) -> i32 {
    let mut ans: i32 = 0;
    for bit in (0..=B).rev() {
        let b = ((val >> bit) & 1) as usize;
        if !t.to[b ^ 1].is_none() {
            ans ^= 1 << bit;
            t = t.to[b ^ 1].as_ref().unwrap();
        } else {
            t = t.to[b].as_ref().unwrap();
        }
    }
    ans
}

fn dfs(
    v: usize,
    children: &Vec<Vec<usize>>,
    queries_for: &Vec<Vec<(i32, usize)>>,
    trie_root: &mut Trie,
    ans: &mut Vec<i32>,
) {
    trie_root.add(v as u32, B);

    for &(val, id) in &queries_for[v] {
        let best = traverse_trie(trie_root, val);
        ans[id] = best;
    }

    for &child in &children[v] {
        dfs(child, children, queries_for, trie_root, ans);
    }

    trie_root.rem(v as u32, B);
}

impl Solution {
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = parents.len();
        let q = queries.len();

        let mut queries_for = vec![Vec::<(i32, usize)>::new(); n];
        for i in 0..q {
            let &[node, val] = &queries[i][..] else {
                panic!("Bad queries format");
            };
            queries_for[node as usize].push((val, i));
        }

        let tree_root = parents.iter().position(|&p| p == -1).unwrap();
        let mut children = vec![Vec::<usize>::new(); n];
        for i in 0..n {
            let p = parents[i];
            if p == -1 {
                continue;
            }
            children[p as usize].push(i);
        }

        let mut ans = vec![0; q];
        let mut trie_root = Trie::new();

        dfs(tree_root, &children, &queries_for, &mut trie_root, &mut ans);

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
    #[case(vec![-1,0,1,1], vec![vec![0,2],vec![3,2],vec![2,5]], vec![2,3,7])]
    #[case(vec![3,7,-1,2,0,7,0,2], vec![vec![4,6],vec![1,15],vec![0,5]], vec![6,14,7])]
    fn case(#[case] parents: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_genetic_difference(parents, queries);
        assert_eq!(actual, expected);
    }
}
