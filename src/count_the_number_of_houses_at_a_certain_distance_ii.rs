//! Solution for https://leetcode.com/problems/count-the-number-of-houses-at-a-certain-distance-ii
//! 3017. Count the Number of Houses at a Certain Distance II

struct Arr {
    sum_a0: Vec<i64>,
    sum_d: Vec<i64>,
    a: Vec<i64>,
}

impl Arr {
    fn new(n: usize) -> Self {
        Self {
            sum_a0: vec![0; n + 1],
            sum_d: vec![0; n + 1],
            a: vec![0; n],
        }
    }

    // [l, r] are 1-indexed
    fn add_seg(&mut self, mut l: usize, r: usize, a0: i64, d: i64) {
        assert!(l >= 1);
        l -= 1;

        // now [l; r) in 0-indexed

        // for i in l..r {
        //     self.a[i] += a0 + (i - l) as i64 * d;
        // }
        // return;

        self.sum_d[l] += d;
        self.sum_d[r] -= d;

        let diff = -(l as i64) * d + a0;
        self.sum_a0[l] += diff;
        self.sum_a0[r] -= diff;
    }

    fn get_arr(&self) -> Vec<i64> {
        // return self.a.clone();

        let n = self.sum_a0.len() - 1;
        let mut a0 = vec![0; n];
        let mut d = vec![0; n];
        let mut pref_a0 = 0;
        let mut pref_d = 0;
        for i in 0..n {
            pref_a0 += self.sum_a0[i];
            pref_d += self.sum_d[i];
            a0[i] = pref_a0;
            d[i] = pref_d;
        }

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = a0[i] + i as i64 * d[i];
        }
        a
    }
}

fn div_floor(a: i64, b: i64) -> i64 {
    if b < 0 {
        return div_floor(-a, -b);
    }
    assert!(b > 0);
    if a >= 0 || a % b == 0 {
        return a / b;
    }
    a / b - 1
}

impl Solution {
    pub fn count_of_pairs(n: i32, mut x: i32, mut y: i32) -> Vec<i64> {
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        let n = n as i64;
        let x = x as i64;
        let y = y as i64;
        assert!(x <= y);

        let mut arr = Arr::new(n as usize);
        // let mut ans = vec![0; n as usize];

        let mut process = |L: i64, R: i64, C1: i64, C2: i64| {
            if L > R {
                return;
            }

            // for D in 1..=n {
            //     let l = L.max(D + C2);
            //     let r = R.min(-D + C1);
            //     let cur = r - l + 1;
            //     if cur > 0 {
            //         ans[(D - 1) as usize] += cur;
            //     }
            // }
            // return;

            // all: D >= 1, D <= n

            // 1.
            // L >= D + C2
            // R <= -D + C1
            {
                let rD = (L - C2).min(-R + C1).min(n);
                let lD = 1;
                if lD <= rD {
                    arr.add_seg(lD as usize, rD as usize, R - L + 1, 0);
                }
            }

            // 2.
            // L < D + C2
            // R <= -D + C1
            {
                let lD = (L - C2 + 1).max(1);
                let rD = (-R + C1).min(R - C2).min(n);
                if lD <= rD {
                    let a0 = R - (lD + C2) + 1;
                    let d = -1;
                    assert!(a0 >= 0);
                    arr.add_seg(lD as usize, rD as usize, a0, d);
                }
            }

            // 3.
            // L < D + C2
            // -D + C1 < R
            {
                let lD = (L - C2 + 1).max(-R + C1 + 1).max(1);
                let rD = n.min(div_floor(C1 - C2, 2));
                if lD <= rD {
                    let a0 = (-lD + C1) - (lD + C2) + 1;
                    let d = -2;
                    assert!(a0 >= 0);
                    arr.add_seg(lD as usize, rD as usize, a0, d);
                }
            }

            // 4.
            // D + C2 <= L
            // -D + C1 < R
            {
                let lD = (-R + C1 + 1).max(1);
                let rD = (L - C2).min(-L + C1).min(n);
                let a0 = (-lD + C1) - L + 1;
                let d = -1;
                assert!(a0 >= 0);
                if lD <= rD {
                    arr.add_seg(lD as usize, rD as usize, a0, d);
                }
            }
        };

        for a in 1..=n {
            // b <= -D + C1
            // b >= D + C2
            // b <= R
            // b >= L

            // 1. b <= x
            {
                let L = a + 1;
                let R = x;
                let C2 = a;
                let mut C1 = i64::MAX;

                C1 = C1.min(x + 1 + (a - y).abs());
                C1 = C1.min(y + 1 + (a - x).abs());

                process(L, R, C1, C2);
            }

            // 2. x < b <= y
            {
                let L = (x + 1).max(a + 1);
                let R = y;
                let C2 = a.max(x - 1 - (a - y).abs());
                let C1 = y + 1 + (a - x).abs();

                process(L, R, C1, C2);
            }

            // 3. y < b
            {
                let L = (y + 1).max(a + 1);
                let R = n;
                let mut C2 = a;
                let C1 = 2 * n;

                C2 = C2.max(x - 1 - (a - y).abs());
                C2 = C2.max(y - 1 - (a - x).abs());

                process(L, R, C1, C2);
            }
        }

        // ans.fill(0);
        // for a in 1..=n {
        //     for b in 1..=n {
        //         for D in 1..=n {
        //             let mut ok = true;
        //             ok &= b >= a + 1;
        //             ok &= b <= n;
        //             ok &= b - a >= D;
        //             ok &= (b - x).abs() + (a - y).abs() + 1 >= D;
        //             ok &= (b - y).abs() + (a - x).abs() + 1 >= D;
        //             if ok {
        //                 ans[(D - 1) as usize] += 1;
        //             }
        //         }
        //     }
        // }

        let mut ans = arr.get_arr();
        for i in 1..ans.len() {
            ans[i - 1] -= ans[i];
        }
        for i in 0..ans.len() {
            ans[i] *= 2;
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
    #[case(3, 1, 3, vec![6,0,0])]
    #[case(5, 2, 4, vec![10,8,2,0,0])]
    #[case(4, 1, 1, vec![6,4,2,0])]
    fn case(#[case] n: i32, #[case] x: i32, #[case] y: i32, #[case] expected: Vec<i64>) {
        let actual = Solution::count_of_pairs(n, x, y);
        assert_eq!(actual, expected);
    }
}
