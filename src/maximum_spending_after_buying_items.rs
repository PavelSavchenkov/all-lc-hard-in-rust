//! Solution for https://leetcode.com/problems/maximum-spending-after-buying-items
//! 2931. Maximum Spending After Buying Items

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut all = Vec::<i32>::new();
        for row in &values {
            all.extend(row.iter());
        }
        all.sort();

        let mut ans: i64 = 0;
        for i in 0..all.len() {
            ans += (i + 1) as i64 * all[i] as i64;
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
    #[case(vec![vec![8,5,2],vec![6,4,1],vec![9,7,3]], 285)]
    #[case(vec![vec![10,8,6,4,2],vec![9,7,5,3,2]], 386)]
    fn case(#[case] values: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::max_spending(values);
        assert_eq!(actual, expected);
    }
}
