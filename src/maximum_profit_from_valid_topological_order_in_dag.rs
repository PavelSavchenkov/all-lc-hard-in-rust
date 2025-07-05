//! Solution for https://leetcode.com/problems/maximum-profit-from-valid-topological-order-in-dag
//! 3530. Maximum Profit from Valid Topological Order in DAG

fn bit(mask: usize, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

impl Solution {
    pub fn max_profit(n: i32, edges: Vec<Vec<i32>>, score: Vec<i32>) -> i32 {
        let n = n as usize;

        let mut mask_before = vec![0; n];
        for e in &edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            mask_before[to] |= 1 << from;
        }

        let mut dp = vec![i32::MIN; 1 << n];
        dp[0] = 0;
        for mask in 0..1 << n {
            let cur_dp = dp[mask];
            if cur_dp == i32::MIN {
                continue;
            }
            for next in 0..n {
                if !bit(mask, next) && (mask_before[next] & mask) == mask_before[next] {
                    let nmask = mask | (1 << next);
                    dp[nmask] =
                        dp[nmask].max(cur_dp + (mask.count_ones() as i32 + 1) * score[next]);
                }
            }
        }
        dp[(1 << n) - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![0,1]], vec![2,3], 8)]
    #[case(3, vec![vec![0,1],vec![0,2]], vec![1,6,3], 25)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] score: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_profit(n, edges, score);
        assert_eq!(actual, expected);
    }
}
