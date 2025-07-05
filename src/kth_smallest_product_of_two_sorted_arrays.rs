//! Solution for https://leetcode.com/problems/kth-smallest-product-of-two-sorted-arrays
//! 2040. Kth Smallest Product of Two Sorted Arrays

const MAX_VAL: i64 = 1_00_000;

fn divide_floor(a: i64, b: i64) -> i64 {
    assert!(b != 0);
    let mut q = a / b;
    let r = a % b;
    if r != 0 && a.signum() == -b.signum() {
        q -= 1;
    }
    q
}

fn divide_ceil(a: i64, b: i64) -> i64 {
    assert!(b != 0);
    let mut q = a / b;
    let r = a % b;
    if r != 0 && a.signum() == b.signum() {
        q += 1;
    }
    q
}

impl Solution {
    pub fn kth_smallest_product(a: Vec<i32>, b: Vec<i32>, k: i64) -> i64 {
        let a: Vec<_> = a.iter().map(|&x| x as i64).collect();
        let b: Vec<_> = b.iter().map(|&x| x as i64).collect();
        let n = a.len();
        let m = b.len();

        let cnt_less_or_eq = |x| -> i64 {
            let mut ans: i64 = 0;
            let mut y_neg = m;
            let mut y_pos = m;
            for &num in &a {
                /*
                 num * y <= x
                 1) num == 0
                 2) num > 0
                 y <= floor[x / num]
                 3) num < 0
                 y >= ceil[x / num]
                */
                match num.signum() {
                    0 => {
                        if 0 <= x {
                            ans += m as i64;
                        }
                    }
                    1 => {
                        let lim = divide_floor(x, num);
                        // shrink
                        while y_pos > 0 && b[y_pos - 1] > lim {
                            y_pos -= 1;
                        }
                        // extend
                        while y_pos < m && b[y_pos] <= lim {
                            y_pos += 1;
                        }
                        ans += y_pos as i64;
                    }
                    _ => {
                        assert!(num < 0);
                        let lim = divide_ceil(x, num);
                        // shrink
                        while y_neg > 0 && b[m - y_neg] < lim {
                            y_neg -= 1;
                        }
                        // extend
                        while y_neg < m && b[m - y_neg - 1] >= lim {
                            y_neg += 1;
                        }
                        ans += y_neg as i64;
                    }
                }
            }
            ans
        };

        // smallest x such that there are k sums <= x
        let mut L = -MAX_VAL * MAX_VAL - 1;
        let mut R = MAX_VAL * MAX_VAL;
        while L + 1 != R {
            let M = (L + R) / 2;
            let cur_cnt = cnt_less_or_eq(M);
            if cur_cnt >= k {
                R = M;
            } else {
                L = M;
            }
        }

        R
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,5], vec![3,4], 2, 8)]
    #[case(vec![-4,-2,0,3], vec![2,4], 6, 0)]
    #[case(vec![-2,-1,0,1,2], vec![-3,-1,2,4,5], 3, -6)]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k: i64,
        #[case] expected: i64,
    ) {
        let actual = Solution::kth_smallest_product(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
