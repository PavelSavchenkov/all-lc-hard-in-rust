//! Solution for https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii
//! 1751. Maximum Number of Events That Can Be Attended II

struct Event {
    start: u32,
    end: u32,
    value: u32,
}

impl Event {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            start: v[0] as u32,
            end: v[1] as u32,
            value: v[2] as u32,
        }
    }
}

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut es: Vec<_> = events.into_iter().map(|v| Event::new(&v)).collect();

        es.sort_by_key(|e| e.end);

        let mut link = vec![0; es.len()];
        for i in 0..es.len() {
            link[i] = es.partition_point(|e| e.end < es[i].start);
        }

        let mut dp = vec![0; es.len()];
        for i in 0..es.len() {
            dp[i] = es[i].value;
        }
        for it in 1..k {
            for i in 1..es.len() {
                let prev = dp[i - 1];
                dp[i] = dp[i].max(prev);
            }
            for i in (0..es.len()).rev() {
                let j = link[i];
                if j > 0 {
                    let prev_dp = dp[j - 1];
                    dp[i] = dp[i].max(prev_dp + es[i].value);
                }
            }
        }
        let ans = *dp.iter().max().unwrap();
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,4],vec![3,4,3],vec![2,3,1]], 2, 7)]
    #[case(vec![vec![1,2,4],vec![3,4,3],vec![2,3,10]], 2, 10)]
    #[case(vec![vec![1,1,1],vec![2,2,2],vec![3,3,3],vec![4,4,4]], 3, 9)]
    fn case(#[case] events: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_value(events, k);
        assert_eq!(actual, expected);
    }
}
