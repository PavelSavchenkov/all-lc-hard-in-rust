//! Solution for https://leetcode.com/problems/longest-increasing-subsequence-ii
//! 2407. Longest Increasing Subsequence II

struct SegmTreeMax {
    t: Vec<i64>,
    sz: usize,
}

impl SegmTreeMax {
    fn new(n: usize, val: i64) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            sz,
            t: vec![val; sz * 2],
        }
    }

    fn assign_point(&mut self, pos: usize, val: i64) {
        let mut v = self.sz + pos;
        self.t[v] = val;
        v /= 2;
        while v >= 1 {
            self.t[v] = self.t[v * 2].max(self.t[v * 2 + 1]);
            v /= 2;
        }
    }

    // [l; r)
    fn get_max(&self, mut l: usize, mut r: usize) -> i64 {
        let mut mx = i64::MIN;
        if l >= r {
            return mx;
        }
        r -= 1;
        l += self.sz;
        r += self.sz;
        while l <= r {
            mx = mx.max(self.t[l]);
            mx = mx.max(self.t[r]);
            l = (l + 1) / 2;
            r = (r - 1) / 2;
        }
        mx
    }
}

impl Solution {
    pub fn length_of_lis(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();

        let mut tree = SegmTreeMax::new(vals.len(), 0);
        let mut ans = 0;
        for i in 0..n {
            let lower = vals.partition_point(|&x| x < a[i] - k);
            let upper = vals.binary_search(&a[i]).unwrap();
            let mx = tree.get_max(lower, upper).max(0);

            let cur_dp = tree.get_max(upper, upper + 1);
            if mx + 1 > cur_dp {
                tree.assign_point(upper, mx + 1);
                ans = ans.max(mx + 1);
            }
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
    #[case(vec![4,2,1,4,3,4,5,8,15], 3, 5)]
    #[case(vec![7,4,5,1,8,12,4,7], 5, 4)]
    #[case(vec![1,5], 1, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::length_of_lis(nums, k);
        assert_eq!(actual, expected);
    }
}
