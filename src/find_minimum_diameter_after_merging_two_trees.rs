//! Solution for https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees
//! 3203. Find Minimum Diameter After Merging Two Trees

fn dfs(
    v: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    depth: usize,
    max_depth: &mut usize,
    max_v: &mut usize,
) {
    if depth > *max_depth {
        *max_depth = depth;
        *max_v = v;
    }

    for &to in &g[v] {
        if to == p {
            continue;
        }
        dfs(to, v, g, depth + 1, max_depth, max_v);
    }
}

fn get_far(g: &Vec<Vec<usize>>, root: usize) -> (usize, usize) {
    let mut max_depth = 0;
    let mut max_v = root;
    dfs(root, root, g, 0, &mut max_depth, &mut max_v);
    (max_depth, max_v)
}

fn diam(es: Vec<Vec<i32>>) -> i32 {
    let n = es.len() + 1;
    let mut g = vec![Vec::new(); n];
    for e in &es {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }

    let (_, a) = get_far(&g, 0);
    let (d, _) = get_far(&g, a);
    d as i32
}

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = diam(edges1);
        let d2 = diam(edges2);
        let mut ans = (d1 + 1) / 2 + (d2 + 1) / 2 + 1;
        ans = ans.max(d1).max(d2);
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
    #[case(vec![vec![0,1],vec![0,2],vec![0,3]], vec![vec![0,1]], 3)]
    #[case(vec![vec![0,1],vec![0,2],vec![0,3],vec![2,4],vec![2,5],vec![3,6],vec![2,7]], vec![vec![0,1],vec![0,2],vec![0,3],vec![2,4],vec![2,5],vec![3,6],vec![2,7]], 5)]
    fn case(#[case] edges1: Vec<Vec<i32>>, #[case] edges2: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(actual, expected);
    }
}
