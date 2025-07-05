//! Solution for https://leetcode.com/problems/maximum-performance-of-a-team
//! 1383. Maximum Performance of a Team

use std::collections::BTreeSet;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by_key(|&i| -efficiency[i]);

        let mut ans = 0;
        let mut set = BTreeSet::new();
        let mut set_sum = 0;
        for &i in &ord {
            let cur = efficiency[i] as u64 * (set_sum + speed[i] as u64);
            ans = ans.max(cur);

            set.insert((speed[i], i));
            set_sum += speed[i] as u64;
            if set.len() == k {
                let (_, id) = set.pop_first().unwrap();
                set_sum -= speed[id] as u64;
            }
        }
        (ans % 1_000_000_007) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, vec![2,10,3,1,5,8], vec![5,4,3,9,7,2], 2, 60)]
    #[case(6, vec![2,10,3,1,5,8], vec![5,4,3,9,7,2], 3, 68)]
    #[case(6, vec![2,10,3,1,5,8], vec![5,4,3,9,7,2], 4, 72)]
    fn case(
        #[case] n: i32,
        #[case] speed: Vec<i32>,
        #[case] efficiency: Vec<i32>,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_performance(n, speed, efficiency, k);
        assert_eq!(actual, expected);
    }
}
