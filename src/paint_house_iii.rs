//! Solution for https://leetcode.com/problems/paint-house-iii
//! 1473. Paint House III

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let target_segs = target as usize;
        let houses: Vec<_> = houses.iter().map(|&x| x as usize).collect();

        // started_segs, last_color
        let mut dp = vec![vec![i32::MAX; n]; m + 1];
        dp[0][0] = 0;
        for i in 0..m {
            let mut ndp = vec![vec![i32::MAX; n]; m + 1];
            for started_segs in 0..=i {
                for last_color in 0..n {
                    let cur_dp = dp[started_segs][last_color];
                    if cur_dp == i32::MAX {
                        continue;
                    }
                    for cur_color in 0..n {
                        if houses[i] != 0 && houses[i] - 1 != cur_color {
                            continue;
                        }
                        let mut new_started_segs = started_segs;
                        if cur_color != last_color || i == 0 {
                            new_started_segs += 1;
                        }
                        let mut cur_cost = 0;
                        if houses[i] == 0 {
                            cur_cost += cost[i][cur_color];
                        }
                        remin(&mut ndp[new_started_segs][cur_color], cur_dp + cur_cost);
                    }
                }
            }
            dp = ndp;
        }
        let ans = *dp[target_segs].iter().min().unwrap();
        if ans == i32::MAX {
            return -1;
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
    #[case(vec![0,0,0,0,0], vec![vec![1,10],vec![10,1],vec![10,1],vec![1,10],vec![5,1]], 5, 2, 3, 9)]
    #[case(vec![0,2,1,2,0], vec![vec![1,10],vec![10,1],vec![10,1],vec![1,10],vec![5,1]], 5, 2, 3, 11)]
    #[case(vec![3,1,2,3], vec![vec![1,1,1],vec![1,1,1],vec![1,1,1],vec![1,1,1]], 4, 3, 3, -1)]
    fn case(
        #[case] houses: Vec<i32>,
        #[case] cost: Vec<Vec<i32>>,
        #[case] m: i32,
        #[case] n: i32,
        #[case] target: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::min_cost(houses, cost, m, n, target);
        assert_eq!(actual, expected);
    }
}
