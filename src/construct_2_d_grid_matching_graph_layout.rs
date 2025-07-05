//! Solution for https://leetcode.com/problems/construct-2d-grid-matching-graph-layout
//! 3311. Construct 2D Grid Matching Graph Layout

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut s = 0;
        for v in 0..n {
            if g[v].len() < g[s].len() {
                s = v;
            }
        }
        let mut rows = Vec::new();
        if g[s].len() == 1 {
            let mut used = vec![false; n];
            let mut row = Vec::new();
            row.push(s);
            used[s] = true;
            let mut v = s;
            loop {
                let mut next = v;
                for &to in &g[v] {
                    if !used[to] {
                        next = to;
                    }
                }
                if next == v {
                    assert!(g[v].len() == 1);
                    break;
                }
                row.push(next);
                used[next] = true;
                v = next;
            }
            rows.push(row);
        } else {
            assert!(g[s].len() == 2);
            let mut used = vec![false; n];
            let mut row = Vec::new();
            row.push(s);
            used[s] = true;
            let mut v = s;
            loop {
                let mut next = v;
                for &to in &g[v] {
                    if used[to] {
                        continue;
                    }
                    if g[to].len() == 2 {
                        next = to;
                        break;
                    }
                    if g[to].len() == 3 {
                        next = to;
                    }
                }
                row.push(next);
                used[next] = true;
                v = next;
                if g[v].len() == 2 {
                    break;
                }
            }

            rows.push(row.clone());
            assert!(n % row.len() == 0);
            let iters = n / row.len() - 1;
            for it in 0..iters {
                let mut next_row = Vec::new();
                for &v in &row {
                    let mut next = v;
                    for &to in &g[v] {
                        if !used[to] {
                            assert!(next == v);
                            next = to;
                        }
                    }
                    assert!(next != v);
                    next_row.push(next);
                    used[next] = true;
                }
                row = next_row;
                rows.push(row.clone());
            }
        }

        rows.iter()
            .map(|row| row.iter().map(|&x| x as i32).collect())
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![0,1],vec![0,2],vec![1,3],vec![2,3]], vec![vec![3,1],vec![2,0]])]
    #[case(5, vec![vec![0,1],vec![1,3],vec![2,3],vec![2,4]], vec![vec![4,2,3,1,0]])]
    #[case(9, vec![vec![0,1],vec![0,4],vec![0,5],vec![1,7],vec![2,3],vec![2,4],vec![2,5],vec![3,6],vec![4,6],vec![4,7],vec![6,8],vec![7,8]], vec![vec![8,6,3],vec![7,4,2],vec![1,0,5]])]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::construct_grid_layout(n, edges);
        assert_eq!(actual, expected);
    }
}
