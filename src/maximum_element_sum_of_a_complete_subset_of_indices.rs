//! Solution for https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices
//! 2862. Maximum Element-Sum of a Complete Subset of Indices

impl Solution {
    pub fn maximum_sum(a: Vec<i32>) -> i64 {
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let n = a.len();

        let mut norm = vec![1; n + 1];
        for d in 2..=n {
            if d * d > n {
                break;
            }
            for i in (d * d..=n).step_by(d * d) {
                norm[i] = norm[i].max(d * d);
            }
        }
        let mut sum = vec![0 as i64; n];
        for i in 1..=n {
            let dest = i / norm[i];
            sum[dest - 1] += a[i - 1] as i64;
        }
        *sum.iter().max().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![8,7,3,5,7,2,4,9], 16)]
    #[case(vec![8,10,3,8,1,13,7,9,4], 20)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::maximum_sum(nums);
        assert_eq!(actual, expected);
    }
}
