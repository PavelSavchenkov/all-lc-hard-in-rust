//! Solution for https://leetcode.com/problems/maximize-the-minimum-game-score
//! 3449. Maximize the Minimum Game Score

impl Solution {
    pub fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let m = m as i64;
        let a: Vec<_> = points.iter().map(|&x| x as i64).collect();
        let n = a.len();

        let can = |min: i64| -> bool {
            if min <= 0 {
                return true;
            }
            let mut cnt: Vec<_> = (0..n).map(|i| (min + a[i] - 1) / a[i]).collect();
            let mut ops = 1;
            cnt[0] -= 1;
            for i in 0..n {
                let len = n - i;
                assert!(len >= 2);
                let x = cnt[i];
                let y = cnt[i + 1];
                assert!(x >= 0);
                assert!(y >= 0);
                if len == 2 {
                    let cur = (x * 2).max(y * 2 - 1);
                    ops += cur;
                    break;
                }
                ops += 2 * x;
                cnt[i + 1] = (cnt[i + 1] - x - 1).max(0);
                ops += 1;
            }
            ops <= m
        };

        let mut L = 0;
        let mut R = m * a[0] + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                L = M;
            } else {
                R = M;
            }
        }
        L
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,4], 3, 4)]
    #[case(vec![1,2,3], 5, 2)]
    fn case(#[case] points: Vec<i32>, #[case] m: i32, #[case] expected: i64) {
        let actual = Solution::max_score(points, m);
        assert_eq!(actual, expected);
    }
}
