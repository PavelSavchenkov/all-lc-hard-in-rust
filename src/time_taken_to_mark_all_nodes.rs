//! Solution for https://leetcode.com/problems/time-taken-to-mark-all-nodes
//! 3241. Time Taken to Mark All Nodes

fn dfs_dp(g: &Vec<Vec<usize>>, v: usize, p: usize, dp: &mut Vec<i32>) {
    dp[v] = 0;
    for &to in &g[v] {
        if to == p {
            continue;
        }
        dfs_dp(g, to, v, dp);
        let mut sub = dp[to];
        if to % 2 == 1 {
            sub += 1;
        } else {
            sub += 2;
        }
        dp[v] = dp[v].max(sub);
    }
}

fn dfs(g: &Vec<Vec<usize>>, v: usize, p: usize, up_dp: i32, dp: &Vec<i32>, ans: &mut Vec<i32>) {
    let mut mx1 = 0;
    let mut mx2 = 0;
    if p != v {
        assert!(up_dp >= 0);
        let mut sub = up_dp;
        if p % 2 == 1 {
            sub += 1;
        } else {
            sub += 2;
        }
        mx1 = sub;
    }

    for &to in &g[v] {
        if to == p {
            continue;
        }
        let mut sub = dp[to];
        if to % 2 == 1 {
            sub += 1;
        } else {
            sub += 2;
        }
        if sub > mx1 {
            mx2 = mx1;
            mx1 = sub;
        } else if sub > mx2 {
            mx2 = sub;
        }
    }

    ans[v] = mx1;

    for &to in &g[v] {
        if to == p {
            continue;
        }
        let mut sub = dp[to];
        // todo: avoid copy-paste
        if to % 2 == 1 {
            sub += 1;
        } else {
            sub += 2;
        }
        let mx = if sub == mx1 { mx2 } else { mx1 };
        dfs(g, to, v, mx, dp, ans);
    }
}

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut dp = vec![0; n];
        dfs_dp(&g, 0, 0, &mut dp);

        let mut ans = vec![0; n];
        dfs(&g, 0, 0, 0, &dp, &mut ans);

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
    #[case(vec![vec![0,1],vec![0,2]], vec![2,4,3])]
    #[case(vec![vec![0,1]], vec![1,2])]
    #[case(vec![vec![2,4],vec![0,1],vec![2,3],vec![0,2]], vec![4,6,3,5,5])]
    fn case(#[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::time_taken(edges);
        assert_eq!(actual, expected);
    }
}
