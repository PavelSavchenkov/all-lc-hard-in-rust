//! Solution for https://leetcode.com/problems/maximum-number-of-robots-within-budget
//! 2398. Maximum Number of Robots Within Budget

use std::collections::VecDeque;

fn can(charge_times: &Vec<i32>, running_costs: &Vec<i32>, len: usize, budget: i64) -> bool {
    let n = charge_times.len();
    let mut deque = VecDeque::new();
    let mut sum: i64 = 0;

    for i in 0..n {
        while !deque.is_empty() && charge_times[*deque.back().unwrap()] <= charge_times[i] {
            deque.pop_back();
        }
        deque.push_back(i);
        sum += running_costs[i] as i64;

        if i >= len {
            sum -= running_costs[i - len] as i64;
            if *deque.front().unwrap() == i - len {
                deque.pop_front();
            }
        }

        if i + 1 >= len && charge_times[*deque.front().unwrap()] as i64 + len as i64 * sum <= budget
        {
            return true;
        }
    }

    false
}

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = charge_times.len();

        let mut L = 0;
        let mut R = n + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(&charge_times, &running_costs, M, budget) {
                L = M
            } else {
                R = M;
            }
        }
        L as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,6,1,3,4], vec![2,1,3,4,5], 25, 3)]
    #[case(vec![11,12,19], vec![10,8,7], 19, 0)]
    fn case(
        #[case] charge_times: Vec<i32>,
        #[case] running_costs: Vec<i32>,
        #[case] budget: i64,
        #[case] expected: i32,
    ) {
        let actual = Solution::maximum_robots(charge_times, running_costs, budget);
        assert_eq!(actual, expected);
    }
}
