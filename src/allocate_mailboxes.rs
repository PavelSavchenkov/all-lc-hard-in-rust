//! Solution for https://leetcode.com/problems/allocate-mailboxes
//! 1478. Allocate Mailboxes

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = houses.len();

        houses.sort();

        let mut best_placement = vec![vec![i32::MAX; n]; n];
        for l in 0..n {
            for r in l..n {
                let med = (l + r) / 2;
                let mut sum = 0;
                for i in l..=r {
                    sum += (houses[i] - houses[med]).abs();
                }
                best_placement[l][r] = sum;
            }
        }

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for done_boxes in 0..k {
            let mut ndp = dp.clone();
            for i in 0..n {
                let cur_dp = dp[i];
                if cur_dp == i32::MAX {
                    continue;
                }
                for j in i + 1..=n {
                    remin(&mut ndp[j], cur_dp + best_placement[i][j - 1]);
                }
            }
            dp = ndp;
        }

        dp[n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4,8,10,20], 3, 5)]
    #[case(vec![2,3,5,12,18], 2, 9)]
    fn case(#[case] houses: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_distance(houses, k);
        assert_eq!(actual, expected);
    }
}
