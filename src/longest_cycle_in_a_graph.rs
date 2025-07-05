//! Solution for https://leetcode.com/problems/longest-cycle-in-a-graph
//! 2360. Longest Cycle in a Graph

fn dfs(
    v: usize,
    es: &Vec<i32>,
    when: &mut Vec<i32>,
    color: &mut Vec<i32>,
    timer: &mut i32,
    ans: &mut i32,
) {
    color[v] = 1;
    when[v] = *timer;
    *timer += 1;

    let to = es[v];
    if to != -1 {
        let to = to as usize;
        if color[to] == 1 {
            *ans = (*ans).max(when[v] - when[to] + 1);
        } else if color[to] == 0 {
            dfs(to, es, when, color, timer, ans);
        }
    }
    color[v] = 2;
}

impl Solution {
    pub fn longest_cycle(es: Vec<i32>) -> i32 {
        let n = es.len();
        let mut when = vec![-1; n];
        let mut color = vec![0; n];
        let mut timer = 0;
        let mut ans = -1;
        for i in 0..n {
            if color[i] == 0 {
                dfs(i, &es, &mut when, &mut color, &mut timer, &mut ans);
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
    #[case(vec![3,3,4,2,3], 3)]
    #[case(vec![2,-1,3,1], -1)]
    fn case(#[case] edges: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_cycle(edges);
        assert_eq!(actual, expected);
    }
}
