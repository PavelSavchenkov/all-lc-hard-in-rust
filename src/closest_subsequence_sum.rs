//! Solution for https://leetcode.com/problems/closest-subsequence-sum
//! 1755. Closest Subsequence Sum

fn gen_subs(a: &Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut sum = vec![0; 1 << n];
    for mask in 1..((1 << n) as usize) {
        let b = mask.trailing_zeros() as usize;
        sum[mask] = sum[mask ^ (1 << b)] + a[b];
    }
    sum.sort();
    sum.dedup();
    sum
}

impl Solution {
    pub fn min_abs_difference(a: Vec<i32>, goal: i32) -> i32 {
        let n = a.len();

        let half = n / 2;
        let left = gen_subs(&a[0..half].to_vec());
        let right = gen_subs(&a[half..].to_vec());

        let mut ans = goal.abs();
        let mut update_ans = |i: usize, j: usize| {
            if j >= right.len() {
                return;
            }
            let cur = (goal - left[i] - right[j]).abs();
            ans = ans.min(cur);
        };
        let mut j = right.len();
        for i in 0..left.len() {
            while j > 0 && left[i] + right[j - 1] >= goal {
                j -= 1;
            }
            update_ans(i, j);
            if j > 0 {
                update_ans(i, j - 1);
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
    #[case(vec![5,-7,3,5], 6, 0)]
    #[case(vec![7,-9,15,-2], -5, 1)]
    #[case(vec![1,2,3], -7, 7)]
    fn case(#[case] nums: Vec<i32>, #[case] goal: i32, #[case] expected: i32) {
        let actual = Solution::min_abs_difference(nums, goal);
        assert_eq!(actual, expected);
    }
}
