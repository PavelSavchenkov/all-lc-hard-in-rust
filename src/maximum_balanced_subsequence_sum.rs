//! Solution for https://leetcode.com/problems/maximum-balanced-subsequence-sum
//! 2926. Maximum Balanced Subsequence Sum

struct FenwTreeMax {
    t: Vec<i64>,
}

impl FenwTreeMax {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_max(&mut self, r: usize) -> i64 {
        let mut mx = i64::MIN;
        if r == 0 {
            return mx;
        }
        let mut i = r - 1;
        loop {
            mx = mx.max(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        mx
    }
}

impl Solution {
    pub fn max_balanced_subsequence_sum(a: Vec<i32>) -> i64 {
        let n = a.len();

        let mut vals = Vec::new();
        for i in 0..n {
            vals.push(a[i] - i as i32);
        }
        vals.sort();
        vals.dedup();

        let mut ans = i64::MIN;
        let mut tree = FenwTreeMax::new(vals.len(), 0);
        for i in 0..n {
            let val = a[i] - i as i32;
            let pos = vals.binary_search(&val).unwrap();
            let max_prev = tree.get_max(pos + 1);
            let cur_dp = max_prev.max(0) + a[i] as i64;
            tree.relax_point(pos, cur_dp);
            ans = ans.max(cur_dp);
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
    #[case(vec![3,3,5,6], 14)]
    #[case(vec![5,-1,-3,8], 13)]
    #[case(vec![-2,-1], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::max_balanced_subsequence_sum(nums);
        assert_eq!(actual, expected);
    }
}
