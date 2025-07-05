//! Solution for https://leetcode.com/problems/minimum-money-required-before-transactions
//! 2412. Minimum Money Required Before Transactions

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let n = transactions.len();

        let mut bals = Vec::new();
        let mut costs = Vec::new();
        for t in &transactions {
            let cost = t[0];
            let cashback = t[1];
            bals.push((-cost + cashback) as i64);
            costs.push(cost as i64);
        }

        let mut min_sum = 0;
        for i in 0..n {
            if bals[i] < 0 {
                min_sum += bals[i];
            }
        }

        let mut ans = 0;
        for i in 0..n {
            // money + min_sum_w/o_i >= cost[i]

            let mut cur_min_sum = min_sum;
            if bals[i] < 0 {
                cur_min_sum -= bals[i];
            }

            ans = ans.max(costs[i] - cur_min_sum);
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
    #[case(vec![vec![2,1],vec![5,0],vec![4,2]], 10)]
    #[case(vec![vec![3,0],vec![0,3]], 3)]
    fn case(#[case] transactions: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::minimum_money(transactions);
        assert_eq!(actual, expected);
    }
}
