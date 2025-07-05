//! Solution for https://leetcode.com/problems/redundant-connection-ii
//! 685. Redundant Connection II

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        let mut g = vec![Vec::new(); n];
        for i in 0..edges.len() {
            let par = (edges[i][0] - 1) as usize;
            let son = (edges[i][1] - 1) as usize;
            g[son].push((par, i));
        }

        let mut root = None;
        for v in 0..n {
            if g[v].is_empty() {
                assert!(root.is_none());
                root = Some(v);
            }
        }

        let mut ans = 0;
        if root.is_some() {
            // some internal node has 2 outgoing edges
            let root = root.unwrap();
            let mut v2 = n;
            for v in 0..n {
                if g[v].len() == 2 {
                    assert!(v2 == n);
                    v2 = v;
                }
            }
            assert!(v2 < n);
            for it in 0..2 {
                let edge = g[v2].remove(it);

                let mut u = g[v2][0].0;
                let mut found = false;
                for it2 in 0..n {
                    if u == root {
                        found = true;
                        break;
                    }
                    assert!(g[u].len() == 1);
                    u = g[u][0].0;
                }
                if found {
                    ans = ans.max(edge.1);
                }

                g[v2].insert(it, edge);
            }
        } else {
            // there is a cycle and it contains the original root

            let mut st = Vec::new();
            let mut was = vec![false; n];
            let mut v = 0; // any node
            loop {
                assert!(g[v].len() == 1);
                let (u, _) = g[v][0];
                if was[u] {
                    loop {
                        let w: usize = st.pop().unwrap();
                        let id = g[w][0].1;
                        ans = ans.max(id);
                        if w == u {
                            break;
                        }
                    }
                    break;
                }
                was[u] = true;
                st.push(u);
                v = u;
            }
        }

        edges[ans].clone()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![1,3],vec![2,3]], vec![2,3])]
    #[case(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,1],vec![1,5]], vec![4,1])]
    fn case(#[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_redundant_directed_connection(edges);
        assert_eq!(actual, expected);
    }
}
