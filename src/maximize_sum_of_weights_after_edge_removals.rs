//! Solution for https://leetcode.com/problems/maximize-sum-of-weights-after-edge-removals
//! 3367. Maximize Sum of Weights after Edge Removals

fn dfs(v: usize, p: usize, g: &Vec<Vec<(usize, i32)>>, dp: &mut Vec<Vec<i64>>, k: usize) {
    let mut base = 0;
    let mut vals = Vec::new();
    for &(to, w) in &g[v] {
        if to == p {
            continue;
        }
        dfs(to, v, g, dp, k);
        let dp0 = dp[to][0] + w as i64;
        let dp1 = dp[to][1];
        base += dp1;
        vals.push(-dp1 + dp0);
    }
    vals.sort();
    vals.reverse();
    for up_deleted in 0..2 {
        let upper = if up_deleted == 1 { k } else { k - 1 };
        let mut cur = 0;
        for i in 0..upper.min(vals.len()) {
            if vals[i] > 0 {
                cur += vals[i];
            }
        }
        dp[v][up_deleted] = base + cur;
    }
}

impl Solution {
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let k = k as usize;
        let n = edges.len() + 1;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let w = e[2];
            g[a].push((b, w));
            g[b].push((a, w));
        }

        let mut dp = vec![vec![0; 2]; n];
        dfs(0, 0, &g, &mut dp, k);
        dp[0][1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,4],vec![0,2,2],vec![2,3,12],vec![2,4,6]], 2, 22)]
    #[case(vec![vec![0,1,5],vec![1,2,10],vec![0,3,15],vec![3,4,20],vec![3,5,5],vec![0,6,10]], 3, 65)]
    fn case(#[case] edges: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::maximize_sum_of_weights(edges, k);
        assert_eq!(actual, expected);
    }
}
