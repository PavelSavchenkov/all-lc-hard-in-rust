//! Solution for https://leetcode.com/problems/difference-between-maximum-and-minimum-price-sum
//! 2538. Difference Between Maximum and Minimum Price Sum

fn dfs_down(v: usize, p: usize, g: &Vec<Vec<usize>>, price: &Vec<i32>, max_down: &mut Vec<i64>) {
    max_down[v] = price[v] as i64;
    for &to in &g[v] {
        if to != p {
            dfs_down(to, v, g, price, max_down);
            max_down[v] = max_down[v].max(max_down[to] + price[v] as i64);
        }
    }
}

fn dfs(
    v: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    price: &Vec<i32>,
    max_down: &Vec<i64>,
    max_up: i64,
    ans: &mut i64,
) {
    let max = max_down[v].max(max_up + price[v] as i64);
    let min = price[v] as i64;
    *ans = (*ans).max(max - min);

    let mut mx1 = 0;
    let mut mx2 = 0;
    for &to in &g[v] {
        if to == p {
            continue;
        }
        let cur = max_down[to];
        if cur > mx1 {
            mx2 = mx1;
            mx1 = cur;
        } else if cur > mx2 {
            mx2 = cur;
        }
    }

    for &to in &g[v] {
        if to == p {
            continue;
        }
        let mx = if max_down[to] == mx1 { mx2 } else { mx1 };

        let mut new_max_up = max_up.max(mx);
        new_max_up += price[v] as i64;
        dfs(to, v, g, price, max_down, new_max_up, ans);
    }
}

impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let mut max_down = vec![0 as i64; n];
        dfs_down(0, 0, &g, &price, &mut max_down);

        let mut ans: i64 = 0;
        dfs(0, 0, &g, &price, &max_down, 0, &mut ans);

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
    #[case(6, vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4],vec![3,5]], vec![9,8,7,6,10,5], 24)]
    #[case(3, vec![vec![0,1],vec![1,2]], vec![1,1,1], 2)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] price: Vec<i32>,
        #[case] expected: i64,
    ) {
        let actual = Solution::max_output(n, edges, price);
        assert_eq!(actual, expected);
    }
}
