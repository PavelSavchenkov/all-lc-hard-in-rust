//! Solution for https://leetcode.com/problems/minimum-cost-to-reach-destination-in-time
//! 1928. Minimum Cost to Reach Destination in Time

fn remin(a: &mut i32, b: i32) {
    if *a > b {
        *a = b;
    }
}

#[derive(Clone)]
struct Edge {
    to: usize,
    time: usize,
}

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let max_time = max_time as usize;
        let n = passing_fees.len();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let time = e[2] as usize;
            if time > max_time {
                continue;
            }
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(Edge { to: b, time });
            g[b].push(Edge { to: a, time });
        }

        let mut min_fee = vec![vec![i32::MAX; n]; max_time + 1];
        min_fee[0][0] = passing_fees[0];
        for t in 0..max_time {
            for v in 0..n {
                let cur_fee = min_fee[t][v];
                if cur_fee == i32::MAX {
                    continue;
                }
                for e in &g[v] {
                    if t + e.time > max_time {
                        continue;
                    }
                    remin(&mut min_fee[t + e.time][e.to], cur_fee + passing_fees[e.to]);
                }
            }
        }
        let mut ans = i32::MAX;
        for t in 1..=max_time {
            ans = ans.min(min_fee[t][n - 1]);
        }
        if ans == i32::MAX {
            ans = -1;
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
    #[case(30, vec![vec![0,1,10],vec![1,2,10],vec![2,5,10],vec![0,3,1],vec![3,4,10],vec![4,5,15]], vec![5,1,2,20,20,3], 11)]
    #[case(29, vec![vec![0,1,10],vec![1,2,10],vec![2,5,10],vec![0,3,1],vec![3,4,10],vec![4,5,15]], vec![5,1,2,20,20,3], 48)]
    #[case(25, vec![vec![0,1,10],vec![1,2,10],vec![2,5,10],vec![0,3,1],vec![3,4,10],vec![4,5,15]], vec![5,1,2,20,20,3], -1)]
    fn case(
        #[case] max_time: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] passing_fees: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::min_cost(max_time, edges, passing_fees);
        assert_eq!(actual, expected);
    }
}
