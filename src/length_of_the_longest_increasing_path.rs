//! Solution for https://leetcode.com/problems/length-of-the-longest-increasing-path
//! 3288. Length of the Longest Increasing Path

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn calc_dp(ps: &Vec<Point>) -> Vec<usize> {
    let n = ps.len();
    let mut dp_ans = vec![1; n];
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = i32::MIN;
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && ps[i].x == ps[j].x {
            j += 1;
        }

        for it in 0..2 {
            for k in i..j {
                if it == 0 {
                    let len = dp.partition_point(|&last| last < ps[k].y);
                    assert!(len > 0);
                    dp_ans[k] = len;
                } else {
                    let len = dp_ans[k];
                    dp[len] = dp[len].min(ps[k].y);
                }
            }
        }

        i = j;
    }
    dp_ans
}

impl Solution {
    pub fn max_path_length(ps: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ps: Vec<_> = ps.iter().map(|v| Point { x: v[0], y: v[1] }).collect();
        let k = k as usize;
        let p0 = ps[k];

        ps.sort_by_key(|p| p.x);
        let k = ps.iter().position(|&p| p == p0).unwrap();

        let dp_pref = calc_dp(&ps);

        ps.reverse();
        for p in &mut ps {
            p.x = -p.x;
            p.y = -p.y;
        }
        let mut dp_suff = calc_dp(&ps);
        for p in &mut ps {
            p.x = -p.x;
            p.y = -p.y;
        }
        ps.reverse();
        dp_suff.reverse();

        let ans = dp_pref[k] + dp_suff[k] - 1;
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
    #[case(vec![vec![3,1],vec![2,2],vec![4,1],vec![0,0],vec![5,3]], 1, 3)]
    #[case(vec![vec![2,1],vec![7,0],vec![5,6]], 2, 2)]
    fn case(#[case] coordinates: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_path_length(coordinates, k);
        assert_eq!(actual, expected);
    }
}
