//! Solution for https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph
//! 1761. Minimum Degree of a Connected Trio in a Graph

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![false; n]; n];
        let mut deg = vec![0; n];
        for e in &edges {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            g[a][b] = true;
            g[b][a] = true;
            deg[a] += 1;
            deg[b] += 1;
        }

        let mut ans = usize::MAX;
        for a in 0..n {
            for b in 0..a {
                for c in 0..b {
                    if g[a][b] && g[b][c] && g[a][c] {
                        let cur = deg[a] + deg[b] + deg[c] - 6;
                        ans = ans.min(cur);
                    }
                }
            }
        }

        if ans == usize::MAX {
            return -1;
        }
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
    #[case(6, vec![vec![1,2],vec![1,3],vec![3,2],vec![4,1],vec![5,2],vec![3,6]], 3)]
    #[case(7, vec![vec![1,3],vec![4,1],vec![4,3],vec![2,5],vec![5,6],vec![6,7],vec![7,5],vec![2,6]], 0)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_trio_degree(n, edges);
        assert_eq!(actual, expected);
    }
}
