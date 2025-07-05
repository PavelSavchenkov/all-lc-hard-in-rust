//! Solution for https://leetcode.com/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls
//! 1467. Probability of a Two Boxes Having The Same Number of Distinct Balls

impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let balls: Vec<_> = balls.iter().map(|&x| x as usize).collect();
        let max_cnt = *balls.iter().max().unwrap();
        let sum_cnt = balls.iter().sum::<usize>();
        assert!(sum_cnt % 2 == 0);
        let n = sum_cnt / 2;

        let mut binom = vec![vec![0; max_cnt + 1]; 2 * n + 1];
        for i in 0..=2 * n {
            binom[i][0] = 1;
            for j in 1..=i.min(max_cnt) {
                binom[i][j] = binom[i - 1][j] + binom[i - 1][j - 1];
            }
        }

        let max_distinct = balls.len();
        let max_cnt = n;
        // cnt_left, diff_distinct
        let mut dp = vec![vec![0.0; 2 * max_distinct + 1]; n + 1];
        dp[0][max_distinct] = 1.0;
        let mut cnt_seen = 0;
        for i in 0..balls.len() {
            let mut ndp = vec![vec![0.0; 2 * max_distinct + 1]; n + 1];
            for cnt_left in 0..=cnt_seen.min(n) {
                let cnt_right = cnt_seen - cnt_left;
                if cnt_right > n {
                    continue;
                }
                let denom = binom[cnt_seen + balls[i]][balls[i]];
                for to_left in 0..=balls[i] {
                    if cnt_left + to_left > n {
                        continue;
                    }
                    let to_right = balls[i] - to_left;
                    let coef =
                        binom[cnt_left + to_left][to_left] * binom[cnt_right + to_right][to_right];
                    let coef = coef as f64 / denom as f64;
                    for cur_diff in max_distinct - i..=max_distinct + i {
                        let cur_dp = dp[cnt_left][cur_diff];
                        let mut new_diff = cur_diff;
                        if to_left == 0 {
                            new_diff -= 1;
                        } else if to_right == 0 {
                            new_diff += 1;
                        }
                        ndp[cnt_left + to_left][new_diff] += cur_dp * coef;
                    }
                }
            }
            cnt_seen += balls[i];
            dp = ndp;
        }
        dp[n][max_distinct]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1], 1.00000)]
    #[case(vec![2,1,1], 0.66667)]
    #[case(vec![1,2,1,2], 0.60000)]
    fn case(#[case] balls: Vec<i32>, #[case] expected: f64) {
        let actual = Solution::get_probability(balls);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5."
        );
    }
}
