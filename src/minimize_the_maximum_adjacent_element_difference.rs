//! Solution for https://leetcode.com/problems/minimize-the-maximum-adjacent-element-difference
//! 3357. Minimize the Maximum Adjacent Element Difference

impl Solution {
    pub fn min_difference(a: Vec<i32>) -> i32 {
        let n = a.len();

        if *a.iter().max().unwrap() == -1 {
            return 0;
        }

        let can = |d: i32| -> bool {
            if d < 0 {
                return false;
            }
            let mut segs = Vec::new();
            for it in 0..2 {
                // it == 0 --> y - x <= d
                // it == 1 --> y > x + d
                let mut i = 0;
                while i < n {
                    if a[i] >= 0 {
                        if i + 1 < n && a[i + 1] >= 0 && (a[i + 1] - a[i]).abs() > d {
                            return false;
                        }
                        i += 1;
                        continue;
                    }
                    let mut j = i;
                    while j < n && a[j] == -1 {
                        j += 1;
                    }
                    if i == 0 {
                        let x = a[j];
                        segs.push((x - d, x + d));
                    } else if j == n {
                        let x = a[i - 1];
                        segs.push((x - d, x + d));
                    } else if j - i == 1 {
                        let mut one = a[i - 1];
                        let mut two = a[j];
                        if one > two {
                            std::mem::swap(&mut one, &mut two);
                        }
                        let l = two - d;
                        let r = one + d;
                        if l > r {
                            return false;
                        }
                        segs.push((l, r));
                    } else {
                        assert!(i > 0 && j < n && j - i > 1);
                        let mut one = a[i - 1];
                        let mut two = a[j];
                        if one > two {
                            std::mem::swap(&mut one, &mut two);
                        }
                        if it == 0 {
                            segs.push((one - d, one + d));
                            segs.push((two - d, two + d));
                        } else {
                            let l = two - d;
                            let r = one + d;
                            if l > r {
                                return false;
                            }
                            segs.push((l, r));
                        }
                    }
                    i = j;
                }
                if segs.is_empty() {
                    return true;
                }
                segs.sort_by_key(|&s| s.1);
                let x = segs[0].1;
                let mut L = i32::MIN;
                let mut R = i32::MAX;
                for &(l, r) in &segs {
                    if !(l <= x && x <= r) {
                        L = L.max(l);
                        R = R.min(r);
                    }
                }
                // eprintln!("it = {}, L = {}, R = {}, x = {}", it, L, R, x);
                if L > R {
                    continue;
                }
                if it == 1 {
                    return true;
                }
                R = R.min(x + d);
                if L <= R {
                    return true;
                }
            }

            false
        };

        let mut R = 1_000_000_000;
        let mut L = -1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if can(M) {
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
    #[case(vec![1,2,-1,10,8], 4)]
    #[case(vec![-1,-1,-1], 0)]
    #[case(vec![-1,10,-1,8], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_difference(nums);
        assert_eq!(actual, expected);
    }
}
