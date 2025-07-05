//! Solution for https://leetcode.com/problems/minimum-cost-to-equalize-array
//! 3139. Minimum Cost to Equalize Array

fn solve(R: i64, M: i64, n: i64, C1: i64, C2: i64) -> i64 {
    if R >= M {
        let mut ans = i64::MAX;
        for it in 0..2 {
            let r = R + it * (n - 1);
            let m = M + it;
            let mut cur = C2 * ((r + m) / 2);
            if (r + m) % 2 == 1 {
                cur += C1;
            }
            ans = ans.min(cur);
        }
        return ans;
    }

    let mut ans = C2 * R + C1 * (M - R);
    if n == 2 {
        return ans;
    }
    assert!(n > 2);
    let K = (M - R - 1) / (n - 2);
    for k in [K, K + 1, K + 2] {
        let r = R + k * (n - 1);
        let m = M + k;
        if r >= m {
            assert!(k >= K + 1);
            let mut cur = C2 * ((r + m) / 2);
            if (r + m) % 2 == 1 {
                cur += C1;
            }
            ans = ans.min(cur);
        } else {
            assert!(k == K);
            let cur = C2 * r + C1 * (m - r);
            ans = ans.min(cur);
        }
    }
    ans
}

impl Solution {
    pub fn min_cost_to_equalize_array(mut a: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let n = a.len();
        a.sort();

        if n == 1 {
            return 0;
        }

        let mut ans;
        if cost1 * 2 < cost2 {
            ans = 0;
            for i in 0..n - 1 {
                ans += (a[n - 1] - a[i]) as i64 * cost1 as i64;
            }
        } else {
            let M = a[n - 1];
            for i in 0..n {
                a[i] = M - a[i];
            }
            a.reverse();
            let M = a[n - 1] as i64;
            let R = a.iter().fold(0 as i64, |acc, e| acc + *e as i64) - M;
            ans = solve(R, M, n as i64, cost1 as i64, cost2 as i64);
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
    #[case(vec![4,1], 5, 2, 15)]
    #[case(vec![2,3,3,3,5], 2, 1, 6)]
    #[case(vec![3,5,3], 1, 3, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] cost1: i32, #[case] cost2: i32, #[case] expected: i32) {
        let actual = Solution::min_cost_to_equalize_array(nums, cost1, cost2);
        assert_eq!(actual, expected);
    }
}
