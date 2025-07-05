//! Solution for https://leetcode.com/problems/ipo
//! 502. IPO

use std::collections::BTreeSet;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut k = k as usize;
        let n = profits.len();

        let mut by_cap: Vec<_> = (0..n).collect();
        by_cap.sort_by_key(|&i| capital[i]);

        let mut set = BTreeSet::<(i32, usize)>::new();
        let mut ptr = 0;
        while k > 0 {
            while ptr < n && w >= capital[by_cap[ptr]] {
                set.insert((profits[by_cap[ptr]], by_cap[ptr]));
                ptr += 1;
            }
            if set.is_empty() {
                break;
            }
            k -= 1;
            let (profit, _) = set.pop_last().unwrap().clone();
            w += profit;
        }
        w
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 0, vec![1,2,3], vec![0,1,1], 4)]
    #[case(3, 0, vec![1,2,3], vec![0,1,2], 6)]
    fn case(
        #[case] k: i32,
        #[case] w: i32,
        #[case] profits: Vec<i32>,
        #[case] capital: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::find_maximized_capital(k, w, profits, capital);
        assert_eq!(actual, expected);
    }
}
