//! Solution for https://leetcode.com/problems/minimum-cost-to-make-array-equal
//! 2448. Minimum Cost to Make Array Equal

impl Solution {
    pub fn min_cost(a: Vec<i32>, cost: Vec<i32>) -> i64 {
        let n = a.len();
        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| a[i]);

        let mut pref_cost = vec![0 as i64; n + 1];
        let mut pref_mul = vec![0 as i64; n + 1];
        for i in 0..n {
            pref_cost[i + 1] = pref_cost[i] + cost[ord[i]] as i64;
            pref_mul[i + 1] = pref_mul[i] + cost[ord[i]] as i64 * a[ord[i]] as i64;
        }

        let mut ans = i64::MAX;
        for i in 0..n {
            let x = a[ord[i]] as i64;
            let mut cur = 0;
            cur += x * pref_cost[i] - pref_mul[i];
            cur += (pref_mul[n] - pref_mul[i + 1]) - x * (pref_cost[n] - pref_cost[i + 1]);
            ans = ans.min(cur);
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
    #[case(vec![1,3,5,2], vec![2,3,1,14], 8)]
    #[case(vec![2,2,2,2,2], vec![4,2,8,1,3], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] cost: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::min_cost(nums, cost);
        assert_eq!(actual, expected);
    }
}
