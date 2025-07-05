//! Solution for https://leetcode.com/problems/create-components-with-same-value
//! 2440. Create Components With Same Value

fn dfs_par(v: usize, par: &mut Vec<usize>, g: &Vec<Vec<usize>>) {
    for &to in &g[v] {
        if to != par[v] {
            par[to] = v;
            dfs_par(to, par, g);
        }
    }
}

fn check_sum(g: &Vec<Vec<usize>>, nums: &Vec<i32>, sum: i32) -> bool {
    let n = g.len();
    let mut par = vec![n; n];
    dfs_par(0, &mut par, &g);

    let mut st = Vec::new();
    let mut deg_down = vec![0; n];
    for v in 0..n {
        deg_down[v] = g[v].len();
        if v != 0 {
            deg_down[v] -= 1;
        }
        if deg_down[v] == 0 {
            st.push(v);
        }
    }

    let mut sum_comp = nums.clone();
    while !st.is_empty() {
        let v = st.pop().unwrap();
        assert!(deg_down[v] == 0);
        if sum_comp[v] > sum {
            return false;
        }
        if v != 0 {
            deg_down[par[v]] -= 1;
            if deg_down[par[v]] == 0 {
                st.push(par[v]);
            }
        }
        if sum_comp[v] < sum {
            if v == 0 {
                return false;
            }
            sum_comp[par[v]] += sum_comp[v];
        }
    }
    true
}

impl Solution {
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let n = nums.len();

        let mut g = vec![Vec::<usize>::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        for d in 1..=sum {
            if sum % d == 0 {
                if check_sum(&g, &nums, d) {
                    return sum / d - 1;
                }
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![6,2,2,2,6], vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4]], 2)]
    #[case(vec![2], vec![], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::component_value(nums, edges);
        assert_eq!(actual, expected);
    }
}
