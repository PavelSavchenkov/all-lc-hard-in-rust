//! Solution for https://leetcode.com/problems/earliest-possible-day-of-full-bloom
//! 2136. Earliest Possible Day of Full Bloom

struct Plant {
    prep: usize,
    grow: usize,
}

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut ps: Vec<_> = plant_time
            .iter()
            .zip(grow_time.iter())
            .map(|(&prep, &grow)| Plant {
                prep: prep as usize,
                grow: grow as usize,
            })
            .collect();

        ps.sort_by_key(|p| p.grow);
        ps.reverse();

        let can = |T| -> bool {
            let mut sum = 0;
            for p in &ps {
                if T < p.grow + 1 {
                    return false;
                }
                let deadline = T - p.grow - 1;
                sum += p.prep;
                if sum > deadline + 1 {
                    return false;
                }
            }
            true
        };

        let mut L = 0;
        let mut R = 1_000_000_000;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                R = M
            } else {
                L = M
            }
        }

        R as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4,3], vec![2,3,1], 9)]
    #[case(vec![1,2,3,2], vec![2,1,2,1], 9)]
    #[case(vec![1], vec![1], 2)]
    fn case(#[case] plant_time: Vec<i32>, #[case] grow_time: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::earliest_full_bloom(plant_time, grow_time);
        assert_eq!(actual, expected);
    }
}
