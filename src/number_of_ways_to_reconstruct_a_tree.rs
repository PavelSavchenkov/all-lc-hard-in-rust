//! Solution for https://leetcode.com/problems/number-of-ways-to-reconstruct-a-tree
//! 1719. Number Of Ways To Reconstruct A Tree

fn dfs(
    v: usize,
    g: &Vec<Vec<usize>>,
    tin: &mut Vec<usize>,
    tout: &mut Vec<usize>,
    timer: &mut usize,
) {
    tin[v] = *timer;
    for &to in &g[v] {
        dfs(to, g, tin, tout, timer);
    }
    tout[v] = *timer;
    *timer += 1;
}

fn solve(g: &Vec<Vec<usize>>, n: usize) -> i32 {
    let mut by_deg: Vec<_> = (0..n).collect();
    by_deg.sort_by_key(|&v| g[v].len());
    by_deg.reverse();

    let root = by_deg[0];
    if g[root].len() != n - 1 {
        return 0;
    }
    by_deg.remove(0);

    let mut par = vec![n; n];
    let mut depth = vec![n; n];
    depth[root] = 0;
    for &v in &by_deg {
        let mut max_depth = 0;
        for &to in &g[v] {
            if depth[to] < n && depth[to] >= max_depth {
                par[v] = to;
                max_depth = depth[to];
            }
        }
        if par[v] == n {
            return 0;
        }
        depth[v] = depth[par[v]] + 1;
    }

    let mut g_built = vec![Vec::new(); n];
    for v in 0..n {
        if v != root {
            g_built[par[v]].push(v);
        }
    }

    let mut tin = vec![0; n];
    let mut tout = vec![0; n];
    let mut timer = 0;
    dfs(root, &g_built, &mut tin, &mut tout, &mut timer);

    let is_upper = |v: usize, u: usize| -> bool { tin[v] <= tin[u] && tout[u] <= tout[v] };

    let mut g_matr = vec![vec![false; n]; n];
    for v in 0..n {
        for &to in &g[v] {
            g_matr[v][to] = true;
        }
    }

    for v in 0..n {
        for u in 0..n {
            if v == u {
                continue;
            }
            let should = is_upper(v, u) || is_upper(u, v);
            let actual = g_matr[v][u];
            if should != actual {
                return 0;
            }
        }
    }

    for v in 0..n {
        if v != root {
            if g[v].len() == g[par[v]].len() {
                return 2;
            }
        }
    }
    1
}

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut nodes = Vec::new();
        for p in &pairs {
            let a = p[0] as usize;
            let b = p[1] as usize;
            nodes.push(a);
            nodes.push(b);
        }
        nodes.sort();
        nodes.dedup();
        let n = nodes.len();

        let mut g = vec![Vec::new(); n];
        for p in &pairs {
            let a = p[0] as usize;
            let b = p[1] as usize;
            let a = nodes.binary_search(&a).unwrap();
            let b = nodes.binary_search(&b).unwrap();
            g[a].push(b);
            g[b].push(a);
        }

        solve(&g, n)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![2,3]], 1)]
    #[case(vec![vec![1,2],vec![2,3],vec![1,3]], 2)]
    #[case(vec![vec![1,2],vec![2,3],vec![2,4],vec![1,5]], 0)]
    fn case(#[case] pairs: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::check_ways(pairs);
        assert_eq!(actual, expected);
    }
}
