//! Solution for https://leetcode.com/problems/maximum-value-of-k-coins-from-piles
//! 2218. Maximum Value of K Coins From Piles

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = piles.len();

        let mut dp = vec![Vec::new(); n + 1];
        for i in 0..n {
            dp[i] = vec![vec![i32::MIN; k + 1]; piles[i].len() + 1];
        }
        dp[n] = vec![vec![i32::MIN; k + 1]; 1];
        dp[0][0][0] = 0;

        for i in 0..n {
            for j in 0..=piles[i].len() {
                for have in 0..=k {
                    let cur_dp = dp[i][j][have];
                    if cur_dp == i32::MIN {
                        continue;
                    }
                    remax(&mut dp[i + 1][0][have], cur_dp);
                    if j < piles[i].len() && have < k {
                        remax(&mut dp[i][j + 1][have + 1], cur_dp + piles[i][j]);
                    }
                }
            }
        }
        dp[n][0][k]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,100,3],vec![7,8,9]], 2, 101)]
    #[case(vec![vec![100],vec![100],vec![100],vec![100],vec![100],vec![100],vec![1,1,1,1,1,1,700]], 7, 706)]
    fn case(#[case] piles: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_value_of_coins(piles, k);
        assert_eq!(actual, expected);
    }
}
