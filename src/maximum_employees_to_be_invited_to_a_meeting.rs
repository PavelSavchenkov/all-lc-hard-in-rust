//! Solution for https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting
//! 2127. Maximum Employees to Be Invited to a Meeting

fn dfs(
    v: usize,
    f: &Vec<usize>,
    st: &mut Vec<usize>,
    pos_st: &mut Vec<Option<usize>>,
    used: &mut Vec<bool>,
    max_cycle: &mut usize,
) {
    assert!(!used[v]);
    assert!(pos_st[v].is_none());
    used[v] = true;

    pos_st[v] = Some(st.len());
    st.push(v);
    let to = f[v];
    if !pos_st[to].is_none() {
        let cycle_len = pos_st[v].unwrap() - pos_st[to].unwrap() + 1;
        *max_cycle = (*max_cycle).max(cycle_len);
    }
    if !used[to] {
        dfs(to, f, st, pos_st, used, max_cycle);
    }
    st.pop();
    pos_st[v] = None;
}

fn dfs2(v: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<usize>, mx_dp: &mut usize) {
    *mx_dp = dp[v].max(*mx_dp);

    for &to in &g[v] {
        if dp[to] > dp[v] + 1 {
            dp[to] = dp[v] + 1;
            dfs2(to, g, dp, mx_dp);
        }
    }
}

impl Solution {
    pub fn maximum_invitations(f: Vec<i32>) -> i32 {
        let f: Vec<_> = f.iter().map(|&i| i as usize).collect();
        let n = f.len();

        let mut st = Vec::<usize>::new();
        let mut pos_st = vec![None as Option<usize>; n];
        let mut used = vec![false; n];
        let mut max_cycle: usize = 0;
        for v in 0..n {
            if used[v] {
                continue;
            }
            dfs(v, &f, &mut st, &mut pos_st, &mut used, &mut max_cycle);
        }

        let mut g = vec![vec![]; n];
        for i in 0..n {
            g[f[i]].push(i);
        }

        let mut ans_roots = 0;
        let mut dp = vec![usize::MAX; n];
        for i in 0..n {
            if f[f[i]] == i && dp[i] == usize::MAX {
                dp[i] = 0;
                dp[f[i]] = 0;
                let mut mx_dp_i = 0;
                dfs2(i, &g, &mut dp, &mut mx_dp_i);
                let mut mx_dp_fi = 0;
                dfs2(f[i], &g, &mut dp, &mut mx_dp_fi);
                ans_roots += 2 + mx_dp_i + mx_dp_fi;
            }
        }

        let ans = ans_roots.max(max_cycle);
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
    #[case(vec![2,2,1,2], 3)]
    #[case(vec![1,2,0], 3)]
    #[case(vec![3,0,1,4,1], 4)]
    fn case(#[case] favorite: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximum_invitations(favorite);
        assert_eq!(actual, expected);
    }
}
