//! Solution for https://leetcode.com/problems/tree-of-coprimes
//! 1766. Tree of Coprimes

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 && b != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

fn dfs(
    v: usize,
    p: usize,
    depth: i32,
    g: &Vec<Vec<usize>>,
    nums: &Vec<i32>,
    last_with_num: &mut Vec<(i32, usize)>,
    ans: &mut Vec<i32>,
) {
    let x = nums[v] as usize;
    let mut max_depth = -1;
    let mut best_u = -1;
    for y in 1..last_with_num.len() {
        let (dep, u) = last_with_num[y];
        if dep == -1 {
            continue;
        }
        if dep > max_depth && gcd(x, y) == 1 {
            max_depth = dep;
            best_u = u as i32;
        }
    }
    ans[v] = best_u;

    let prev_val = last_with_num[x];
    last_with_num[x] = (depth, v);
    for &to in &g[v] {
        if to == p {
            continue;
        }
        dfs(to, v, depth + 1, g, nums, last_with_num, ans);
    }
    last_with_num[x] = prev_val;
}

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        let M = *nums.iter().max().unwrap() as usize;
        let mut last_with_num = vec![(-1, 0); M + 1];
        let mut ans = vec![-1; n];
        dfs(0, 0, 0, &g, &nums, &mut last_with_num, &mut ans);
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
    #[case(vec![2,3,3,2], vec![vec![0,1],vec![1,2],vec![1,3]], vec![-1,0,0,1])]
    #[case(vec![5,6,10,2,3,6,15], vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]], vec![-1,0,-1,0,0,0,-1])]
    fn case(#[case] nums: Vec<i32>, #[case] edges: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::get_coprimes(nums, edges);
        assert_eq!(actual, expected);
    }
}
