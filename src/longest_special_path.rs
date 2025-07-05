//! Solution for https://leetcode.com/problems/longest-special-path
//! 3425. Longest Special Path

struct State {
    last_with_num: Vec<Option<usize>>, // num --> v
    st: Vec<usize>,
    pos_st: Vec<Option<usize>>, // v --> pos in "st"
    nums: Vec<usize>,
    depth_node: Vec<usize>, // v --> depth in nodes
    depth_len: Vec<i32>,    // v --> depth in edges' weights
    longest_path: i32,
    min_nodes: usize,
}

impl State {
    fn new(nums: Vec<usize>) -> Self {
        let n = nums.len();
        Self {
            last_with_num: vec![None; n],
            st: Vec::new(),
            pos_st: vec![None; n],
            nums,
            depth_node: vec![0; n],
            depth_len: vec![0; n],
            longest_path: -1,
            min_nodes: 0,
        }
    }
}

fn dfs(v: usize, p: usize, mut start_v: usize, g: &Vec<Vec<(usize, i32)>>, state: &mut State) {
    let n = g.len();

    state.pos_st[v] = Some(state.st.len());
    state.st.push(v);

    if let Some(u) = state.last_with_num[state.nums[v]] {
        let pos = state.pos_st[u].unwrap();
        let cand_start = state.st[pos + 1];
        if state.depth_node[cand_start] > state.depth_node[start_v] {
            start_v = cand_start;
        }
    }
    let prev_last = state.last_with_num[state.nums[v]];
    state.last_with_num[state.nums[v]] = Some(v);

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

    state.last_with_num[state.nums[v]] = prev_last;
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
    #[case(vec![vec![0,1,2],vec![1,2,3],vec![1,3,5],vec![1,4,4],vec![2,5,6]], vec![2,1,2,1,3,1], vec![6,2])]
    #[case(vec![vec![1,0,8]], vec![2,2], vec![0,1])]
    fn case(#[case] edges: Vec<Vec<i32>>, #[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::longest_special_path(edges, nums);
        assert_eq!(actual, expected);
    }
}
