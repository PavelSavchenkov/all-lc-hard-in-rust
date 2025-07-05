//! Solution for https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups
//! 2493. Divide Nodes Into the Maximum Number of Groups

fn dfs(v: usize, color: &mut Vec<i32>, g: &Vec<Vec<i32>>) -> bool {
    for to in 0..g.len() {
        if g[v][to] == 1 {
            if color[to] == -1 {
                color[to] = color[v] ^ 1;
                if !dfs(to, color, g) {
                    return false;
                }
            } else {
                if color[to] != color[v] ^ 1 {
                    return false;
                }
            }
        }
    }
    true
}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let INF = i32::MAX / 4;
        let mut g = vec![vec![INF; n]; n];
        for e in &edges {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            g[a][b] = 1;
            g[b][a] = 1;
        }

        let mut color = vec![-1; n];
        for v in 0..n {
            if color[v] == -1 {
                color[v] = 0;
                if !dfs(v, &mut color, &g) {
                    return -1;
                }
            }
        }

        for i in 0..n {
            g[i][i] = 0;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }

        let mut min_v = vec![n; n];
        for u in 0..n {
            for v in 0..n {
                if g[v][u] < INF {
                    min_v[u] = min_v[u].min(v);
                }
            }
        }

        let mut max_for_v = vec![0 as i32; n];
        for i in 0..n {
            for j in 0..n {
                if g[i][j] < INF {
                    let v = min_v[i];
                    assert!(min_v[j] == v);
                    max_for_v[v] = max_for_v[v].max(g[i][j] + 1);
                }
            }
        }
        max_for_v.iter().sum::<i32>()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, vec![vec![1,2],vec![1,4],vec![1,5],vec![2,6],vec![2,3],vec![4,6]], 4)]
    #[case(3, vec![vec![1,2],vec![2,3],vec![3,1]], -1)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::magnificent_sets(n, edges);
        assert_eq!(actual, expected);
    }
}
