//! Solution for https://leetcode.com/problems/maximize-the-minimum-powered-city
//! 2528. Maximize the Minimum Powered City

impl Solution {
    pub fn max_power(a: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as u64).collect();
        let r = r as usize;
        let k = k as u64;

        let can = |lower: u64| -> bool {
            let mut b = a.clone();
            let mut sum: u64 = 0;
            for i in 0..=r {
                sum += b[i];
            }
            let mut spent: u64 = 0;
            for i in 0..n {
                if sum < lower {
                    let last = (i + r).min(n - 1);
                    let need = lower - sum;
                    b[last] += need;
                    spent += need;
                    sum += need;
                }
                if i >= r {
                    sum -= b[i - r];
                }
                if i + r + 1 < n {
                    sum += b[i + r + 1];
                }
            }

            spent <= k
        };

        let mut L = 0;
        let mut R = u64::pow(10, 11);
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
                L = M;
            } else {
                R = M;
            }
        }
        L as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,5,0], 1, 2, 5)]
    #[case(vec![4,4,4,4], 0, 3, 4)]
    fn case(#[case] stations: Vec<i32>, #[case] r: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::max_power(stations, r, k);
        assert_eq!(actual, expected);
    }
}
