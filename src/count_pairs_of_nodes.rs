//! Solution for https://leetcode.com/problems/count-pairs-of-nodes
//! 1782. Count Pairs Of Nodes

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;

        let mut es = HashMap::new();
        let mut deg = vec![0; n];
        for e in &edges {
            let mut a = (e[0] - 1) as usize;
            let mut b = (e[1] - 1) as usize;
            deg[a] += 1;
            deg[b] += 1;

            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            *es.entry((a, b)).or_insert(0) += 1;
        }

        let mut deg_sorted = deg.clone();
        deg_sorted.sort();

        let mut answers = Vec::new();
        for &q in &queries {
            let mut ans = 0;
            let mut j = n;
            for i in 0..n {
                let pref = deg_sorted[i];
                while j > 0 && deg_sorted[j - 1] + pref > q {
                    j -= 1;
                }
                ans += n - j;
            }
            for i in 0..n {
                if deg[i] + deg[i] > q {
                    ans -= 1;
                }
            }
            assert!(ans % 2 == 0);
            ans /= 2;

            for (&(a, b), &cnt) in &es {
                if deg[a] + deg[b] > q {
                    ans -= 1;
                }
                if deg[a] + deg[b] - cnt > q {
                    ans += 1;
                }
            }
            answers.push(ans as i32);
        }

        answers
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![1,2],vec![2,4],vec![1,3],vec![2,3],vec![2,1]], vec![2,3], vec![6,5])]
    #[case(5, vec![vec![1,5],vec![1,5],vec![3,4],vec![2,5],vec![1,3],vec![5,1],vec![2,3],vec![2,5]], vec![1,2,3,4,5], vec![10,10,9,8,6])]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] queries: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::count_pairs(n, edges, queries);
        assert_eq!(actual, expected);
    }
}
