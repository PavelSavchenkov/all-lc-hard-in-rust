//! Solution for https://leetcode.com/problems/minimum-time-to-finish-the-race
//! 2188. Minimum Time to Finish the Race

use std::u64;

struct Tire {
    f: u64,
    r: u64,
}

impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let tires: Vec<_> = tires
            .iter()
            .map(|t| Tire {
                f: t[0] as u64,
                r: t[1] as u64,
            })
            .collect();
        let change_time = change_time as u64;
        let num_laps = num_laps as usize;

        let mut lap_upper_bound = u64::MAX;
        for t in &tires {
            lap_upper_bound = lap_upper_bound.min(t.f + change_time);
        }

        let mut best_consecutive = Vec::new();
        best_consecutive.push(0);
        for t in &tires {
            let mut lap_cost = t.f;
            let mut sum_cost = lap_cost;
            let mut baseline_sum = lap_upper_bound;
            let mut i = 0;
            while sum_cost <= baseline_sum {
                i += 1;
                if i == best_consecutive.len() {
                    best_consecutive.push(u64::MAX);
                }
                best_consecutive[i] = best_consecutive[i].min(sum_cost);
                lap_cost *= t.r;
                sum_cost += lap_cost;
                baseline_sum += lap_upper_bound;
            }
        }

        let mut dp = vec![u64::MAX; num_laps + 1];
        dp[0] = 0;
        for completed_laps in 0..num_laps {
            let offset = if completed_laps == 0 { 0 } else { change_time };
            for in_a_row in 1..best_consecutive.len() {
                if completed_laps + in_a_row > num_laps {
                    break;
                }
                let more_laps = completed_laps + in_a_row;
                let new_dp = dp[completed_laps] + offset + best_consecutive[in_a_row];
                dp[more_laps] = dp[more_laps].min(new_dp);
            }
        }

        dp[num_laps] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,3],vec![3,4]], 5, 4, 21)]
    #[case(vec![vec![1,10],vec![2,2],vec![3,4]], 6, 5, 25)]
    fn case(
        #[case] tires: Vec<Vec<i32>>,
        #[case] change_time: i32,
        #[case] num_laps: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_finish_time(tires, change_time, num_laps);
        assert_eq!(actual, expected);
    }
}
