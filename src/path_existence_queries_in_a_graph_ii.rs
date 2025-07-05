//! Solution for https://leetcode.com/problems/path-existence-queries-in-a-graph-ii
//! 3534. Path Existence Queries in a Graph II

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        mut nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| nums[i]);

        let mut pos = vec![0; n];
        for i in 0..n {
            pos[ord[i]] = i;
        }

        nums.sort();

        let mut log = 0;
        while (1 << log) <= n {
            log += 1;
        }
        let mut go = vec![vec![0; n]; log];
        for i in 0..n {
            let mut next = nums.partition_point(|&num| num <= nums[i] + max_diff);
            next -= 1;
            go[0][i] = next;
        }
        for l in 1..log {
            for i in 0..n {
                go[l][i] = go[l - 1][go[l - 1][i]];
            }
        }

        let mut ans = Vec::new();
        for q in &queries {
            let mut a = q[0] as usize;
            let mut b = q[1] as usize;
            a = pos[a];
            b = pos[b];
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            let mut cur = 0;
            for l in (0..log).rev() {
                if go[l][a] < b {
                    cur += 1 << l;
                    a = go[l][a];
                }
            }
            if a < b {
                a = go[0][a];
                cur += 1;
            }
            let mut cur = cur as i32;
            if a < b {
                cur = -1;
            }
            ans.push(cur);
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
    #[case(5, vec![1,8,3,4,2], 3, vec![vec![0,3],vec![2,4]], vec![1,1])]
    #[case(5, vec![5,3,1,9,10], 2, vec![vec![0,1],vec![0,2],vec![2,3],vec![4,3]], vec![1,2,-1,1])]
    #[case(3, vec![3,6,1], 1, vec![vec![0,0],vec![0,1],vec![1,2]], vec![0,-1,-1])]
    fn case(
        #[case] n: i32,
        #[case] nums: Vec<i32>,
        #[case] max_diff: i32,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(actual, expected);
    }
}
