//! Solution for https://leetcode.com/problems/add-edges-to-make-degrees-of-all-nodes-even
//! 2508. Add Edges to Make Degrees of All Nodes Even

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            g[a].push(b);
            g[b].push(a);
        }
        let mut vs = Vec::new();
        for v in 0..n {
            g[v].sort();
            if g[v].len() % 2 == 1 {
                vs.push(v);
            }
        }

        let is_edge = |v: usize, u: usize| -> bool {
            let pos = g[v].binary_search(&u);
            pos.is_ok()
        };

        if vs.is_empty() {
            return true;
        }
        if vs.len() % 2 == 1 {
            return false;
        }
        if vs.len() > 4 {
            return false;
        }
        if vs.len() == 2 {
            let a = vs[0];
            let b = vs[1];
            if !is_edge(a, b) {
                return true;
            }
            for v in 0..n {
                if v != a && v != b && !is_edge(v, a) && !is_edge(v, b) {
                    return true;
                }
            }
            return false;
        }
        assert!(vs.len() == 4);

        let a = vs[3];
        vs.pop();
        for it in 0..3 {
            let b = vs[0];
            let c = vs[1];
            let d = vs[2];
            if !is_edge(a, b) && !is_edge(c, d) {
                return true;
            }
            vs.rotate_left(1);
        }

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![1,2],vec![2,3],vec![3,4],vec![4,2],vec![1,4],vec![2,5]], true)]
    #[case(4, vec![vec![1,2],vec![3,4]], true)]
    #[case(4, vec![vec![1,2],vec![1,3],vec![1,4]], false)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::is_possible(n, edges);
        assert_eq!(actual, expected);
    }
}
