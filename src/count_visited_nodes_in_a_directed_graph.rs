//! Solution for https://leetcode.com/problems/count-visited-nodes-in-a-directed-graph
//! 2876. Count Visited Nodes in a Directed Graph

fn dfs(
    v: usize,
    to: &Vec<usize>,
    st: &mut Vec<usize>,
    pos_st: &mut Vec<Option<usize>>,
    ans: &mut Vec<i32>,
) {
    if ans[v] != 0 {
        return;
    }
    assert!(pos_st[v].is_none());
    pos_st[v] = Some(st.len());
    st.push(v);
    if pos_st[to[v]].is_some() {
        let pos = pos_st[to[v]].unwrap();
        let cycle_len = st.len() - pos;
        assert!(cycle_len >= 2);
        for i in pos..st.len() {
            ans[st[i]] = cycle_len as i32;
        }
    } else {
        dfs(to[v], to, st, pos_st, ans);
        if ans[v] == 0 {
            ans[v] = ans[to[v]] + 1;
        }
    }
    st.pop();
    pos_st[v] = None;
}

impl Solution {
    pub fn count_visited_nodes(to: Vec<i32>) -> Vec<i32> {
        let n = to.len();
        let to: Vec<_> = to.iter().map(|&x| x as usize).collect();

        let mut ans = vec![0; n];
        let mut st = Vec::new();
        let mut pos_st = vec![None; n];
        for v in 0..n {
            if ans[v] == 0 {
                dfs(v, &to, &mut st, &mut pos_st, &mut ans);
            }
        }
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
    #[case(vec![1,2,0,0], vec![3,3,3,4])]
    #[case(vec![1,2,3,4,0], vec![5,5,5,5,5])]
    fn case(#[case] edges: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::count_visited_nodes(edges);
        assert_eq!(actual, expected);
    }
}
