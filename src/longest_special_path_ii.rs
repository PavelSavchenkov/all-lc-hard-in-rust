//! Solution for https://leetcode.com/problems/longest-special-path-ii
//! 3486. Longest Special Path II

struct State {
    stack_with_num: Vec<Vec<usize>>, // num --> v0, v1, v2, ..
    st: Vec<usize>,
    pos_st: Vec<Option<usize>>, // v --> pos in "st"
    nums: Vec<usize>,
    depth_node: Vec<usize>, // v --> depth in nodes
    depth_len: Vec<i32>,    // v --> depth in edges' weights
    num2: Option<usize>,    // which num appears two times
    longest_path: i32,
    min_nodes: usize,
}

impl State {
    fn new(nums: Vec<usize>) -> Self {
        let n = nums.len();
        Self {
            stack_with_num: vec![Vec::new(); n],
            st: Vec::new(),
            pos_st: vec![None; n],
            nums,
            depth_node: vec![0; n],
            depth_len: vec![0; n],
            num2: None,
            longest_path: -1,
            min_nodes: 0,
        }
    }
}

fn dfs(v: usize, p: usize, mut start_v: usize, g: &Vec<Vec<(usize, i32)>>, state: &mut State) {
    let n = g.len();

    state.pos_st[v] = Some(state.st.len());
    state.st.push(v);
    let num = state.nums[v];
    let prev_num2 = state.num2;

    if let Some(&u) = state.stack_with_num[num].last() {
        if state.depth_node[u] >= state.depth_node[start_v] {
            if state.num2.is_none() {
                state.num2 = Some(num);
            } else {
                let num2 = state.num2.unwrap();
                let len = state.stack_with_num[num2].len();
                assert!(len >= 2);
                let u2 = state.stack_with_num[num2][len - 2];
                let pos = state.pos_st[u2].unwrap();
                let cand_start2 = state.st[pos + 1];
                let pos = state.pos_st[u].unwrap();
                let cand_start1 = state.st[pos + 1];
                if state.depth_node[cand_start1] > state.depth_node[cand_start2] {
                    start_v = cand_start2;
                    state.num2 = Some(num);
                } else {
                    start_v = cand_start1;
                }
            }
        }
    }
    state.stack_with_num[num].push(v);

    let len = state.depth_len[v] - state.depth_len[start_v];
    let len_node = state.depth_node[v] - state.depth_node[start_v] + 1;
    if len > state.longest_path {
        state.longest_path = len;
        state.min_nodes = len_node;
    }
    if len == state.longest_path && len_node < state.min_nodes {
        state.min_nodes = len_node;
    }

    for &(to, w) in &g[v] {
        if to == p {
            continue;
        }
        state.depth_len[to] = state.depth_len[v] + w;
        state.depth_node[to] = state.depth_node[v] + 1;
        dfs(to, v, start_v, g, state);
    }

    state.num2 = prev_num2;
    state.stack_with_num[num].pop();
    state.pos_st[v] = None;
    state.st.pop();
}

impl Solution {
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        let mut vals = nums.clone();
        vals.sort();
        vals.dedup();
        let nums: Vec<usize> = nums
            .iter()
            .map(|x| vals.binary_search(&x).unwrap())
            .collect();
        let n = nums.len();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let len = e[2];
            g[a].push((b, len));
            g[b].push((a, len));
        }

        let mut state = State::new(nums);
        dfs(0, 0, 0, &g, &mut state);

        vec![state.longest_path, state.min_nodes as i32]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,1],vec![1,2,3],vec![1,3,1],vec![2,4,6],vec![4,7,2],vec![3,5,2],vec![3,6,5],vec![6,8,3]], vec![1,1,0,3,1,2,1,1,0], vec![9,3])]
    #[case(vec![vec![1,0,3],vec![0,2,4],vec![0,3,5]], vec![1,1,0,2], vec![5,2])]
    fn case(#[case] edges: Vec<Vec<i32>>, #[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::longest_special_path(edges, nums);
        assert_eq!(actual, expected);
    }
}
