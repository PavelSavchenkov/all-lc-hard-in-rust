//! Solution for https://leetcode.com/problems/count-number-of-possible-root-nodes
//! 2581. Count Number of Possible Root Nodes

use std::collections::HashSet;

fn dfs_down(
    v: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    guesses: &HashSet<(usize, usize)>,
    cnt_down: &mut Vec<usize>,
) {
    for &to in &g[v] {
        if to == p {
            continue;
        }
        if guesses.contains(&(v, to)) {
            cnt_down[v] += 1;
        }
        dfs_down(to, v, g, guesses, cnt_down);
        cnt_down[v] += cnt_down[to];
    }
}

fn dfs(
    v: usize,
    p: usize,
    good_up: usize,
    k: usize,
    g: &Vec<Vec<usize>>,
    guesses: &HashSet<(usize, usize)>,
    cnt_down: &Vec<usize>,
    ans: &mut usize,
) {
    if good_up + cnt_down[v] >= k {
        *ans += 1;
    }

    for &to in &g[v] {
        if to == p {
            continue;
        }
        let mut new_good_up = good_up;
        new_good_up += cnt_down[v] - cnt_down[to];
        if guesses.contains(&(v, to)) {
            new_good_up -= 1;
        }
        if guesses.contains(&(to, v)) {
            new_good_up += 1;
        }
        dfs(to, v, new_good_up, k, g, guesses, cnt_down, ans);
    }
}

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = edges.len() + 1;
        let k = k as usize;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let guesses = {
            let mut h = HashSet::<(usize, usize)>::new();
            for guess in &guesses {
                let par = guess[0] as usize;
                let son = guess[1] as usize;
                h.insert((par, son));
            }
            h
        };

        let mut cnt_down = vec![0; n];
        dfs_down(0, 0, &g, &guesses, &mut cnt_down);

        let mut ans: usize = 0;
        dfs(0, 0, 0, k, &g, &guesses, &cnt_down, &mut ans);

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1],vec![1,2],vec![1,3],vec![4,2]], vec![vec![1,3],vec![0,1],vec![1,0],vec![2,4]], 3, 3)]
    #[case(vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4]], vec![vec![1,0],vec![3,4],vec![2,1],vec![3,2]], 1, 5)]
    fn case(
        #[case] edges: Vec<Vec<i32>>,
        #[case] guesses: Vec<Vec<i32>>,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::root_count(edges, guesses, k);
        assert_eq!(actual, expected);
    }
}
