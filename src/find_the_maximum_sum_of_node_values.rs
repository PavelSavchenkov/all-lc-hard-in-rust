//! Solution for https://leetcode.com/problems/find-the-maximum-sum-of-node-values
//! 3068. Find the Maximum Sum of Node Values

fn dfs(v: usize, p: usize, g: &Vec<Vec<usize>>, a: &Vec<i32>, k: i32) -> Vec<i64> {
    let mut dp = vec![i64::MIN; 2];
    dp[0] = 0;
    for &to in &g[v] {
        if to == p {
            continue;
        }
        let sub = dfs(to, v, g, a, k);
        let mut ndp = vec![0; 2];
        for my in 0..2 {
            for his in 0..2 {
                if sub[his] == i64::MIN {
                    continue;
                }

                ndp[my] = ndp[my].max(dp[my] + sub[his]);

                let mut cand = dp[my] + sub[his];
                let mut diff = a[to] - (a[to] ^ k);
                if his == 0 {
                    diff = -diff;
                }
                cand += diff as i64;
                ndp[my ^ 1] = ndp[my ^ 1].max(cand);
            }
        }
        dp = ndp;
    }
    dp[0] += a[v] as i64;
    if dp[1] > i64::MIN {
        dp[1] += (a[v] ^ k) as i64;
    }
    dp
}

impl Solution {
    pub fn maximum_value_sum(a: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = a.len();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let dp = dfs(0, 0, &g, &a, k);
        let ans = *dp.iter().max().unwrap();
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
    #[case(vec![1,2,1], 3, vec![vec![0,1],vec![0,2]], 6)]
    #[case(vec![2,3], 7, vec![vec![0,1]], 9)]
    #[case(vec![7,7,7,7,7,7], 3, vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5]], 42)]
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] expected: i64,
    ) {
        let actual = Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(actual, expected);
    }
}
