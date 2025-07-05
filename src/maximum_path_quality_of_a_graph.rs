//! Solution for https://leetcode.com/problems/maximum-path-quality-of-a-graph
//! 2065. Maximum Path Quality of a Graph

fn go(
    g: &Vec<Vec<(usize, usize)>>,
    n: usize,
    max_time: usize,
    values: &Vec<usize>,
    v: usize,
    time_spent: usize,
    mut sum_values: usize,
    used: &mut Vec<bool>,
    ans: &mut usize,
) {
    if time_spent > max_time {
        return;
    }
    if v != 0 && time_spent + 10 > max_time {
        return;
    }
    let old_used_v = used[v];
    if !used[v] {
        sum_values += values[v];
        used[v] = true;
    }
    if v == 0 {
        *ans = (*ans).max(sum_values);
    }
    for &(to, time) in &g[v] {
        go(
            g,
            n,
            max_time,
            values,
            to,
            time_spent + time,
            sum_values,
            used,
            ans,
        );
    }
    used[v] = old_used_v;
}

impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let max_time = max_time as usize;
        let values: Vec<usize> = values.iter().map(|&x| x as usize).collect();

        let mut g = vec![Vec::<(usize, usize)>::new(); n];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let time = e[2] as usize;
            g[u].push((v, time));
            g[v].push((u, time));
        }

        let mut used = vec![false; n];
        let mut ans = 0;
        go(&g, n, max_time, &values, 0, 0, 0, &mut used, &mut ans);
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
    #[case(vec![0,32,10,43], vec![vec![0,1,10],vec![1,2,15],vec![0,3,10]], 49, 75)]
    #[case(vec![5,10,15,20], vec![vec![0,1,10],vec![1,2,10],vec![0,3,10]], 30, 25)]
    #[case(vec![1,2,3,4], vec![vec![0,1,10],vec![1,2,11],vec![2,3,12],vec![1,3,13]], 50, 7)]
    fn case(
        #[case] values: Vec<i32>,
        #[case] edges: Vec<Vec<i32>>,
        #[case] max_time: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::maximal_path_quality(values, edges, max_time);
        assert_eq!(actual, expected);
    }
}
