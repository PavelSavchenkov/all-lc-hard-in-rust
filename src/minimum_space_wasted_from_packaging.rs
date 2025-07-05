//! Solution for https://leetcode.com/problems/minimum-space-wasted-from-packaging
//! 1889. Minimum Space Wasted From Packaging

impl Solution {
    pub fn min_wasted_space(mut packages: Vec<i32>, mut boxes: Vec<Vec<i32>>) -> i32 {
        let n = packages.len();
        packages.sort();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + packages[i] as i64;
        }

        let mut ans = i64::MAX;
        for supplier in 0..boxes.len() {
            let row = &mut boxes[supplier];
            row.sort();

            let mut wasted_space = 0;
            let mut prev_pos = 0;
            for size in row {
                let l = prev_pos;
                let r = packages.partition_point(|&p| p <= *size);
                // [l; r)
                let mut cur = (r - l) as i64 * *size as i64;
                cur -= pref[r] - pref[l];
                wasted_space += cur;

                prev_pos = r;
            }
            if prev_pos == n {
                ans = ans.min(wasted_space);
            }
        }
        if ans == i64::MAX {
            return -1;
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
    #[case(vec![2,3,5], vec![vec![4,8],vec![2,8]], 6)]
    #[case(vec![2,3,5], vec![vec![1,4],vec![2,3],vec![3,4]], -1)]
    #[case(vec![3,5,8,10,11,12], vec![vec![12],vec![11,9],vec![10,5,14]], 9)]
    fn case(#[case] packages: Vec<i32>, #[case] boxes: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_wasted_space(packages, boxes);
        assert_eq!(actual, expected);
    }
}
