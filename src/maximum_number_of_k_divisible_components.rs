//! Solution for https://leetcode.com/problems/maximum-number-of-k-divisible-components
//! 2872. Maximum Number of K-Divisible Components

fn dfs(v: usize, p: usize, g: &Vec<Vec<usize>>, values: &Vec<i32>, k: i32, ans: &mut usize) -> i32 {
    let mut sum = values[v] % k;
    for &to in &g[v] {
        if to != p {
            sum += dfs(to, v, g, values, k, ans);
            sum %= k;
        }
    }
    if sum == 0 {
        *ans += 1;
    }
    sum
}

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let n = n as usize;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut ans = 0;
        dfs(0, 0, &g, &values, k, &mut ans);
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
    #[case(5, vec![vec![0,2],vec![1,2],vec![1,3],vec![2,4]], vec![1,8,1,4,4], 6, 2)]
    #[case(7, vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]], vec![3,0,6,1,5,2,1], 3, 3)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] values: Vec<i32>,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_k_divisible_components(n, edges, values, k);
        assert_eq!(actual, expected);
    }
}
