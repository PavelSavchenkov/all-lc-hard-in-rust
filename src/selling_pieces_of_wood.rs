//! Solution for https://leetcode.com/problems/selling-pieces-of-wood
//! 2312. Selling Pieces of Wood

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;

        let mut price = vec![vec![0; n + 1]; m + 1];
        for p in &prices {
            let h = p[0] as usize;
            let w = p[1] as usize;
            price[h][w] = p[2];
        }

        let mut dp = vec![vec![0 as i64; n + 1]; m + 1];
        for s in 2..=n + m {
            for h in 1..s.min(m + 1) {
                if s <= h {
                    break;
                }
                let w = s - h;
                if w > n {
                    continue;
                }
                let mut ndp = price[h][w] as i64;
                for hh in 1..h {
                    ndp = ndp.max(dp[hh][w] + dp[h - hh][w]);
                }
                for ww in 1..w {
                    ndp = ndp.max(dp[h][ww] + dp[h][w - ww]);
                }
                dp[h][w] = ndp;
            }
        }
        dp[m][n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 5, vec![vec![1,4,2],vec![2,2,7],vec![2,1,3]], 19)]
    #[case(4, 6, vec![vec![3,2,10],vec![1,4,2],vec![4,1,3]], 32)]
    fn case(#[case] m: i32, #[case] n: i32, #[case] prices: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::selling_wood(m, n, prices);
        assert_eq!(actual, expected);
    }
}
