//! Solution for https://leetcode.com/problems/painting-the-walls
//! 2742. Painting the Walls

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        assert!(time.len() == n);

        let mut dp = vec![vec![i32::MAX; n + 1]; n + 1];
        dp[0][0] = 0;
        for i in 0..n {
            let T = time[i] as usize;
            for walls in (0..n).rev() {
                for t in 0..n {
                    let cur_dp = dp[t][walls];
                    if cur_dp == i32::MAX {
                        continue;
                    }
                    let tt = (t + T).min(n);
                    dp[tt][walls + 1] = dp[tt][walls + 1].min(cur_dp + cost[i]);
                }
            }
        }

        let mut ans = i32::MAX;
        for t in 0..=n {
            for walls in n - t..=n {
                let paid = dp[t][walls];
                ans = ans.min(paid);
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
    #[case(vec![1,2,3,2], vec![1,2,3,2], 3)]
    #[case(vec![2,3,4,2], vec![1,1,1,1], 4)]
    fn case(#[case] cost: Vec<i32>, #[case] time: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::paint_walls(cost, time);
        assert_eq!(actual, expected);
    }
}
