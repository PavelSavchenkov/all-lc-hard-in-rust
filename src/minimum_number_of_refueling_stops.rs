//! Solution for https://leetcode.com/problems/minimum-number-of-refueling-stops
//! 871. Minimum Number of Refueling Stops

fn remax(a: &mut i64, b: i64) {
    if *a < b {
        *a = b;
    }
}

struct Station {
    pos: i32,
    fuel: i32,
}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut stations: Vec<_> = stations
            .iter()
            .map(|v| Station {
                pos: v[0],
                fuel: v[1],
            })
            .collect();
        stations.insert(0, Station { pos: 0, fuel: 0 });
        stations.push(Station {
            pos: target,
            fuel: 0,
        });
        let n = stations.len();

        let mut dp = vec![vec![-1 as i64; n + 1]; n + 1];
        dp[0][0] = start_fuel as i64;
        for i in 0..n - 1 {
            for taken in 0..=i {
                let cur_dp = dp[i][taken];
                if cur_dp < 0 {
                    continue;
                }
                let dist_next = stations[i + 1].pos - stations[i].pos;

                // skip current station
                let mut ndp = cur_dp - dist_next as i64;
                remax(&mut dp[i + 1][taken], ndp);
                // take current station
                ndp += stations[i].fuel as i64;
                remax(&mut dp[i + 1][taken + 1], ndp);
            }
        }
        for taken in 0..n - 1 {
            let cur_dp = dp[n - 1][taken];
            if cur_dp >= 0 {
                return taken as i32;
            }
        }
        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1, vec![], 0)]
    #[case(100, 1, vec![vec![10,100]], -1)]
    #[case(100, 10, vec![vec![10,60],vec![20,30],vec![30,30],vec![60,40]], 2)]
    fn case(
        #[case] target: i32,
        #[case] start_fuel: i32,
        #[case] stations: Vec<Vec<i32>>,
        #[case] expected: i32,
    ) {
        let actual = Solution::min_refuel_stops(target, start_fuel, stations);
        assert_eq!(actual, expected);
    }
}
