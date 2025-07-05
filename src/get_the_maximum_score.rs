//! Solution for https://leetcode.com/problems/get-the-maximum-score
//! 1537. Get the Maximum Score

fn remax(a: &mut i64, b: i64) -> bool {
    if *a < b {
        *a = b;
        return true;
    }
    false
}

impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let a = vec![nums1, nums2];

        let mut lower_bound = vec![Vec::new(); 2];
        for it in 0..2 {
            let mut j = 0;
            for i in 0..a[it].len() {
                while j < a[it ^ 1].len() && a[it ^ 1][j] < a[it][i] {
                    j += 1;
                }
                lower_bound[it].push(j);
            }
        }

        let mut order = Vec::new();
        for it in 0..2 {
            for i in 0..a[it].len() {
                order.push((it, i));
            }
        }
        order.sort_by_key(|&(it, i)| a[it][i]);
        for it in 0..2 {
            order.push((it, a[it].len()));
        }

        let mut dp = vec![Vec::new(); 2];
        for it in 0..2 {
            dp[it] = vec![0; a[it].len() + 1];
            dp[it][0] = a[it][0] as i64;
        }

        let mut ans = 0;
        for &(it, i) in &order {
            let cur_dp = dp[it][i];
            if i == a[it].len() {
                ans = ans.max(cur_dp);
                continue;
            }
            // move to another array
            {
                let j = lower_bound[it][i];
                if j < a[it ^ 1].len() && a[it ^ 1][j] == a[it][i] {
                    let mut add = 0;
                    if j + 1 < a[it ^ 1].len() {
                        add += a[it ^ 1][j + 1] as i64;
                    }
                    remax(&mut dp[it ^ 1][j + 1], cur_dp + add);
                }
            }
            // continue current array
            {
                let mut add = 0;
                if i + 1 < a[it].len() {
                    add += a[it][i + 1] as i64;
                }
                remax(&mut dp[it][i + 1], cur_dp + add);
            }
        }
        (ans % 1_000_000_007) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,4,5,8,10], vec![4,6,8,9], 30)]
    #[case(vec![1,3,5,7,9], vec![3,5,100], 109)]
    #[case(vec![1,2,3,4,5], vec![6,7,8,9,10], 40)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_sum(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
