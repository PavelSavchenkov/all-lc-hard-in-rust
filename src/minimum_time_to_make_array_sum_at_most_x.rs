//! Solution for https://leetcode.com/problems/minimum-time-to-make-array-sum-at-most-x
//! 2809. Minimum Time to Make Array Sum At Most x

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}
struct Item {
    a: i32,
    b: i32,
}

impl Solution {
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let n = nums1.len();
        let mut items = Vec::with_capacity(n);
        for i in 0..n {
            items.push(Item {
                a: nums1[i],
                b: nums2[i],
            });
        }
        items.sort_by_key(|i| i.b);

        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            for taken in 0..=i {
                let cur_dp = dp[i][taken];
                // skip
                remax(&mut dp[i + 1][taken], cur_dp);
                // take
                remax(
                    &mut dp[i + 1][taken + 1],
                    cur_dp + items[i].a + items[i].b * (taken as i32 + 1),
                );
            }
        }

        let mut A = 0;
        let mut B = 0;
        for i in 0..n {
            A += items[i].a;
            B += items[i].b;
        }

        for t in 0..=n {
            let all = A + t as i32 * B;
            let cur = all - dp[n][t];
            if cur <= x {
                return t as i32;
            }
        }
        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![1,2,3], 4, 3)]
    #[case(vec![1,2,3], vec![3,3,3], 4, -1)]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] x: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_time(nums1, nums2, x);
        assert_eq!(actual, expected);
    }
}
