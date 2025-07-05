//! Solution for https://leetcode.com/problems/string-transformation
//! 2851. String Transformation

fn calc_z(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut l = 0;
    let mut r = 0;
    let mut z = vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[i + z[i]] == s[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}

fn mult(a: &Vec<Vec<Num>>, b: &Vec<Vec<Num>>) -> Vec<Vec<Num>> {
    let n = a.len();
    let mut c = vec![vec![Num::new(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn power(mut a: Vec<Vec<Num>>, mut p: u64) -> Vec<Vec<Num>> {
    let n = a.len();
    let mut res = vec![vec![Num::new(0); n]; n];
    for i in 0..n {
        res[i][i] = Num::new(1);
    }
    while p > 0 {
        if p % 2 == 1 {
            res = mult(&res, &a);
        }
        a = mult(&a, &a);
        p /= 2;
    }
    res
}

impl Solution {
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        let s = to_u8(&s);
        let t = to_u8(&t);
        let n = s.len();

        let mut S = t.clone();
        S.push(b'$');
        S.extend(s.iter());
        S.extend(s.iter());

        let z = calc_z(&S);
        let mut cnt_ok_cyclic_shifts = 0;
        for i in n + 1..n + 1 + n {
            if z[i] == n {
                cnt_ok_cyclic_shifts += 1;
            }
        }
        if cnt_ok_cyclic_shifts == 0 {
            return 0;
        }
        if cnt_ok_cyclic_shifts == n {
            let ans = Num::new((n - 1) as u32).pow(k as u64);
            return ans.val as i32;
        }
        let C = cnt_ok_cyclic_shifts as u32;

        let mut F = vec![vec![Num::new(0); 2]; 2];
        F[0][0] = Num::new(C - 1);
        F[0][1] = Num::new(C);
        F[1][0] = Num::new(n as u32 - C);
        F[1][1] = Num::new(n as u32 - C - 1);
        F = power(F.clone(), k as u64);
        let ans = if z[n + 1] == n { F[0][0] } else { F[0][1] };
        ans.val as i32
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const MOD: u32 = 1_000_000_007;
type Num = NumMod<MOD>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct NumMod<const M: u32> {
    val: u32,
}

impl<const M: u32> NumMod<M> {
    // doesn't work
    const static_assert: () = {
        assert!(M < (1 << 31));
    };

    fn new(x: u32) -> Self {
        NumMod { val: x % M }
    }

    // x and y are already normalized
    fn safe_add_mod(mut x: u32, y: u32) -> u32 {
        x += y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_sub_mod(mut x: u32, y: u32) -> u32 {
        x += M - y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_mul_mod(x: u32, y: u32) -> u32 {
        ((x as u64 * y as u64) % M as u64) as u32
    }

    fn safe_div_mod(x: u32, y: u32) -> u32 {
        Self::safe_mul_mod(x, Self::inv_mod(y))
    }

    fn pow(&self, p: u64) -> NumMod<M> {
        NumMod::new(Self::pow_mod(self.val, p))
    }

    fn pow_mod(mut x: u32, mut p: u64) -> u32 {
        let mut ans = 1;
        while p > 0 {
            if p % 2 == 1 {
                ans = Self::safe_mul_mod(ans, x);
            }
            x = Self::safe_mul_mod(x, x);
            p /= 2;
        }
        ans
    }

    fn inv_mod(x: u32) -> u32 {
        assert!(x != 0);
        Self::pow_mod(x, (MOD - 2) as u64)
    }
}

impl<const M: u32> Add for NumMod<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_add_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> AddAssign for NumMod<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const M: u32> Sub for NumMod<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_sub_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> SubAssign for NumMod<M> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const M: u32> Mul for NumMod<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_mul_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> MulAssign for NumMod<M> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<const M: u32> Div for NumMod<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_div_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> DivAssign for NumMod<M> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcd", "cdab", 2, 2)]
    #[case("ababab", "ababab", 1, 2)]
    fn case(#[case] s: String, #[case] t: String, #[case] k: i64, #[case] expected: i32) {
        let actual = Solution::number_of_ways(s, t, k);
        assert_eq!(actual, expected);
    }
}
