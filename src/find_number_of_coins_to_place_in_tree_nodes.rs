//! Solution for https://leetcode.com/problems/find-number-of-coins-to-place-in-tree-nodes
//! 2973. Find Number of Coins to Place in Tree Nodes

#[derive(Default)]
struct Node {
    pos: Vec<i32>,
    neg: Vec<i32>,
}

impl Node {
    fn new(x: i32) -> Self {
        let mut this = Self::default();
        this.add(x);
        this
    }

    fn add(&mut self, x: i32) {
        if x > 0 {
            Self::add_vec(&mut self.pos, x, true);
        } else if x < 0 {
            Self::add_vec(&mut self.neg, x, false);
        }
    }

    fn add_vec(vec: &mut Vec<i32>, x: i32, rev: bool) {
        vec.push(x);
        vec.sort();
        if rev {
            vec.reverse();
        }
        while vec.len() > 3 {
            vec.pop();
        }
    }

    fn get_prod(&self) -> Option<i64> {
        let mut all = self.pos.clone();
        all.extend(self.neg.iter());
        if all.len() < 3 {
            return None;
        }
        let mut mx = 0;
        for i in 0..all.len() {
            for j in 0..i {
                for k in 0..j {
                    let cur = all[i] as i64 * all[j] as i64 * all[k] as i64;
                    mx = mx.max(cur);
                }
            }
        }
        Some(mx)
    }

    fn merge(&mut self, other: &Self) {
        for &x in &other.pos {
            self.add(x);
        }
        for &x in &other.neg {
            self.add(x);
        }
    }
}

fn dfs(v: usize, p: usize, g: &Vec<Vec<usize>>, cost: &Vec<i32>, ans: &mut Vec<i64>) -> Node {
    let mut set = Node::new(cost[v]);
    for &to in &g[v] {
        if to == p {
            continue;
        }
        let node = dfs(to, v, g, cost, ans);
        set.merge(&node);
    }
    let mx = set.get_prod();
    if let Some(val) = mx {
        ans[v] = val;
    } else {
        ans[v] = 1;
    }
    set
}

impl Solution {
    pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
        let n = cost.len();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut ans = vec![0; n];
        dfs(0, 0, &g, &cost, &mut ans);

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
    #[case(vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5]], vec![1,2,3,4,5,6], vec![120,1,1,1,1,1])]
    #[case(vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![1,5],vec![2,6],vec![2,7],vec![2,8]], vec![1,4,2,3,5,7,8,-4,2], vec![280,140,32,1,1,1,1,1,1])]
    #[case(vec![vec![0,1],vec![0,2]], vec![1,2,-2], vec![0,1,1])]
    fn case(#[case] edges: Vec<Vec<i32>>, #[case] cost: Vec<i32>, #[case] expected: Vec<i64>) {
        let actual = Solution::placed_coins(edges, cost);
        assert_eq!(actual, expected);
    }
}
