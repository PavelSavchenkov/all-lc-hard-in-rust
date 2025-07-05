//! Solution for https://leetcode.com/problems/maximum-score-of-a-node-sequence
//! 2242. Maximum Score of a Node Sequence

impl Solution {
    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = scores.len();

        let mut g = vec![Vec::<usize>::new(); n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }

        for a in 0..n {
            g[a].sort_by_key(|&v| -scores[v]);
        }

        let mut ans = -1;
        for a in 0..n {
            for &b in &g[a] {
                let base_score = scores[a] + scores[b];

                for i in 0..g[a].len().min(3) {
                    let c = g[a][i];
                    if c == b {
                        continue;
                    }
                    for j in 0..g[b].len().min(3) {
                        let d = g[b][j];
                        if d == c || d == a {
                            continue;
                        }
                        let cur_score = base_score + scores[c] + scores[d];
                        ans = ans.max(cur_score);
                    }
                }
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
    #[case(vec![5,2,9,8,4], vec![vec![0,1],vec![1,2],vec![2,3],vec![0,2],vec![1,3],vec![2,4]], 24)]
    #[case(vec![9,20,6,4,11,12], vec![vec![0,3],vec![5,3],vec![2,4],vec![1,3]], -1)]
    fn case(#[case] scores: Vec<i32>, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximum_score(scores, edges);
        assert_eq!(actual, expected);
    }
}
