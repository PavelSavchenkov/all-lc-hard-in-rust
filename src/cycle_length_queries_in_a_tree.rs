//! Solution for https://leetcode.com/problems/cycle-length-queries-in-a-tree
//! 2509. Cycle Length Queries in a Tree

impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();

        for q in &queries {
            let mut a = q[0];
            let mut b = q[1];

            let mut len = 0;
            while a != b {
                if a > b {
                    len += 1;
                    a /= 2;
                } else {
                    len += 1;
                    b /= 2;
                }
            }
            ans.push(len + 1);
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
    #[case(3, vec![vec![5,3],vec![4,7],vec![2,3]], vec![4,5,3])]
    #[case(2, vec![vec![1,2]], vec![2])]
    fn case(#[case] n: i32, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::cycle_length_queries(n, queries);
        assert_eq!(actual, expected);
    }
}
