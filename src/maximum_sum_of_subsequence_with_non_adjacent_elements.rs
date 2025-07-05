//! Solution for https://leetcode.com/problems/maximum-sum-of-subsequence-with-non-adjacent-elements
//! 3165. Maximum Sum of Subsequence With Non-adjacent Elements

#[derive(Clone)]
struct Node {
    dp: Vec<Vec<i64>>,
}

impl Node {
    fn new(x: i32) -> Self {
        let mut dp = vec![vec![i64::MIN; 2]; 2];
        dp[0][0] = 0;
        dp[1][1] = x as i64;
        Self { dp }
    }

    fn merge(&self, other: &Self) -> Self {
        let mut dp = vec![vec![i64::MIN; 2]; 2];
        for l in 0..2 {
            for m1 in 0..2 {
                for m2 in 0..2 {
                    if m1 == 1 && m2 == 1 {
                        continue;
                    }
                    for r in 0..2 {
                        let left = self.dp[l][m1];
                        let right = other.dp[m2][r];
                        if left > i64::MIN && right > i64::MIN {
                            dp[l][r] = dp[l][r].max(left + right);
                        }
                    }
                }
            }
        }

        Self { dp }
    }

    fn get_max(&self) -> i64 {
        let mut mx = 0;
        for l in 0..2 {
            for r in 0..2 {
                mx = mx.max(self.dp[l][r]);
            }
        }
        mx
    }
}

struct SegmTree {
    t: Vec<Node>,
    sz: usize,
}

impl SegmTree {
    fn new(a: &Vec<i32>) -> Self {
        let mut sz = 1;
        while sz < a.len() {
            sz *= 2;
        }
        let mut t = vec![Node::new(-1); sz * 2];
        for i in 0..a.len() {
            t[sz + i] = Node::new(a[i]);
        }
        let mut this = Self { t, sz };
        for v in (1..sz).rev() {
            this.update_node(v);
        }
        this
    }

    fn update_node(&mut self, v: usize) {
        self.t[v] = self.t[v * 2].merge(&self.t[v * 2 + 1]);
    }

    fn update(&mut self, pos: usize, val: i32) {
        self.update_rec(1, 0, self.sz, pos, val);
    }

    fn update_rec(&mut self, v: usize, tl: usize, tr: usize, pos: usize, val: i32) {
        if tl + 1 == tr {
            self.t[v] = Node::new(val);
            return;
        }

        let tm = (tl + tr) / 2;
        if pos < tm {
            self.update_rec(v * 2, tl, tm, pos, val);
        } else {
            self.update_rec(v * 2 + 1, tm, tr, pos, val);
        }

        self.update_node(v);
    }

    fn get_max(&self) -> i64 {
        self.t[1].get_max()
    }
}

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut tree = SegmTree::new(&nums);
        let mut ans = 0;
        for q in &queries {
            let pos = q[0] as usize;
            let val = q[1];
            tree.update(pos, val);
            let res = tree.get_max();
            ans = (ans + res) % 1_000_000_007 as i64;
        }
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
    #[case(vec![3,5,9], vec![vec![1,-2],vec![0,-3]], 21)]
    #[case(vec![0,-1], vec![vec![0,-5]], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximum_sum_subsequence(nums, queries);
        assert_eq!(actual, expected);
    }
}
