//! Solution for https://leetcode.com/problems/critical-connections-in-a-network
//! 1192. Critical Connections in a Network

fn dfs(
    v: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    depth: &mut Vec<i32>,
    fup: &mut Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
) {
    assert!(depth[v] != -1);
    fup[v] = depth[v];
    for &to in &g[v] {
        if to == p {
            continue;
        }
        if depth[to] != -1 {
            fup[v] = fup[v].min(depth[to]);
        } else {
            depth[to] = depth[v] + 1;
            dfs(to, v, g, depth, fup, ans);
            if fup[to] > depth[v] {
                ans.push(vec![v as i32, to as i32]);
            } else {
                fup[v] = fup[v].min(fup[to]);
            }
        }
    }
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g = vec![Vec::new(); n];
        for e in &connections {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut ans = Vec::new();
        let mut depth = vec![-1; n];
        let mut fup = vec![n as i32; n];
        depth[0] = 0;
        dfs(0, 0, &g, &mut depth, &mut fup, &mut ans);
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
    #[case(4, vec![vec![0,1],vec![1,2],vec![2,0],vec![1,3]], vec![vec![1,3]])]
    #[case(2, vec![vec![0,1]], vec![vec![0,1]])]
    fn case(#[case] n: i32, #[case] connections: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::critical_connections(n, connections);
        assert_eq!(actual, expected);
    }
}
