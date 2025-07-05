//! Solution for https://leetcode.com/problems/count-beautiful-numbers
//! 3490. Count Beautiful Numbers

fn solve(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let mut digs = Vec::new();
    let mut nn = n;
    while nn > 0 {
        let dig = nn % 10;
        digs.push(dig as usize);
        nn /= 10;
    }
    digs.reverse();

    let mut max_dig_sum = 0;
    max_dig_sum += digs[0];
    for i in 1..digs.len() {
        max_dig_sum += 9;
    }
    let max_dig_sum = max_dig_sum as usize;

    let mut ans = 0;
    // dp[less][was_non_zero][prod_dig % m][sum_dig]
    for m in 1..=max_dig_sum {
        let mut dp = vec![vec![vec![vec![0; max_dig_sum + 1]; m]; 2]; 2];
        dp[0][0][1 % m][0] = 1;
        for i in 0..digs.len() {
            let mut ndp = vec![vec![vec![vec![0; max_dig_sum + 1]; m]; 2]; 2];
            for less in 0..2 {
                for was_non_zero in 0..2 {
                    for prod_dig in 0..m {
                        for sum_dig in 0..=max_dig_sum {
                            let cur_dp = dp[less][was_non_zero][prod_dig][sum_dig];
                            if cur_dp == 0 {
                                continue;
                            }
                            for dig in 0..10 {
                                if less == 0 && dig > digs[i] {
                                    continue;
                                }
                                let mut new_less = less;
                                if dig < digs[i] {
                                    new_less = 1;
                                }
                                let mut new_was_non_zero = was_non_zero;
                                if dig != 0 {
                                    new_was_non_zero = 1;
                                }
                                let mut new_prod = prod_dig;
                                if new_was_non_zero == 1 {
                                    new_prod = (prod_dig * dig) % m;
                                }
                                let new_sum = sum_dig + dig;
                                ndp[new_less][new_was_non_zero][new_prod][new_sum] += cur_dp;
                            }
                        }
                    }
                }
            }
            dp = ndp;
        }
        for less in 0..2 {
            ans += dp[less][1][0][m];
        }
    }
    ans
}

impl Solution {
    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        solve(r) - solve(l - 1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(10, 20, 2)]
    #[case(1, 15, 10)]
    fn case(#[case] l: i32, #[case] r: i32, #[case] expected: i32) {
        let actual = Solution::beautiful_numbers(l, r);
        assert_eq!(actual, expected);
    }
}
