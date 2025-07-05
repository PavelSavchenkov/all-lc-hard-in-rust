//! Solution for https://leetcode.com/problems/minimum-cost-to-hire-k-workers
//! 857. Minimum Cost to Hire K Workers

use std::collections::BTreeSet;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let n = quality.len();
        assert!(wage.len() == n);

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by(|&i, &j| {
            // wage[i] / quality[i] < ...
            (wage[i] * quality[j]).cmp(&(wage[j] * quality[i]))
        });

        // todo: use only integers
        let mut set = BTreeSet::new();
        let mut sum_set = 0;
        let mut ans = f64::MAX;
        for &i in &ord {
            if set.len() + 1 >= k {
                let coef = wage[i] as f64 / quality[i] as f64;
                let cur = (sum_set as f64 + quality[i] as f64) * coef;
                ans = ans.min(cur);
            }
            set.insert((quality[i], i));
            sum_set += quality[i];
            if set.len() == k {
                let (q, _) = set.pop_last().unwrap();
                sum_set -= q;
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
    #[case(vec![10,20,5], vec![70,50,30], 2, 105.00000)]
    #[case(vec![3,1,10,10,1], vec![4,8,2,2,7], 3, 30.66667)]
    fn case(
        #[case] quality: Vec<i32>,
        #[case] wage: Vec<i32>,
        #[case] k: i32,
        #[case] expected: f64,
    ) {
        let actual = Solution::mincost_to_hire_workers(quality, wage, k);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5."
        );
    }
}
