//! Solution for https://leetcode.com/problems/abbreviating-the-product-of-a-range
//! 2117. Abbreviating the Product of a Range

const BASE: i64 = 1_00_000_000;

struct Poly {
    c: Vec<i64>,
}

impl Poly {
    fn new(val: i64) -> Self {
        let mut this = Self { c: vec![val] };
        this.normalize();
        this
    }

    fn normalize(&mut self) {
        let mut i = 0;
        let mut carry = 0;
        while i < self.c.len() || carry > 0 {
            if i == self.c.len() {
                self.c.push(0);
            }
            self.c[i] += carry;
            carry = self.c[i] / BASE;
            self.c[i] %= BASE;
            i += 1;
        }
        while self.c.len() > 1 && *self.c.last().unwrap() == 0 {
            self.c.pop();
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut res = vec![0; self.c.len() + other.c.len() - 1];
        for i in 0..self.c.len() {
            for j in 0..other.c.len() {
                res[i + j] += self.c[i] * other.c[j];
            }
        }
        let mut res = Self { c: res };
        res.normalize();
        res
    }

    fn to_string(&self) -> Vec<u8> {
        let mut res = Vec::<u8>::new();
        for i in (0..self.c.len()).rev() {
            let digit = self.c[i];
            let padding = if i + 1 == self.c.len() { 0 } else { 8 };
            let digit_str = format!("{:0padding$}", digit, padding = padding);
            res.extend_from_slice(digit_str.as_bytes());
        }
        res
    }
}

fn get_prod(vals: &Vec<i64>, l: usize, r: usize) -> Poly {
    if r - l == 1 {
        return Poly::new(vals[l]);
    }
    let m = (l + r) / 2;
    let left = get_prod(vals, l, m);
    let right = get_prod(vals, m, r);
    let prod = left.mul(&right);
    prod
}

impl Solution {
    pub fn abbreviate_product(left: i32, right: i32) -> String {
        let left = left as i64;
        let right = right as i64;

        let mut pw2 = 0;
        let mut pw5 = 0;
        for x in left..=right {
            let mut xx = x;
            while xx % 2 == 0 {
                pw2 += 1;
                xx /= 2;
            }
            while xx % 5 == 0 {
                pw5 += 1;
                xx /= 5;
            }
        }
        let C = pw5.min(pw2);
        let mut div5 = C;
        let mut div2 = C;
        let mut vals: Vec<_> = (left..=right).collect();
        for val in &mut vals {
            while div5 > 0 && *val % 5 == 0 {
                *val /= 5;
                div5 -= 1;
            }
            while div2 > 0 && *val % 2 == 0 {
                *val /= 2;
                div2 -= 1;
            }
        }
        assert!(div2 == 0);
        assert!(div5 == 0);

        let prod = get_prod(&vals, 0, vals.len());
        let mut prod_str = prod.to_string();

        if prod_str.len() > 10 {
            let mut prod_str_reduced = Vec::<u8>::new();
            prod_str_reduced.extend_from_slice(&prod_str[0..5]);
            prod_str_reduced.extend_from_slice(b"...");
            prod_str_reduced.extend_from_slice(&prod_str[prod_str.len() - 5..prod_str.len()]);
            prod_str = prod_str_reduced;
        }

        prod_str.extend_from_slice(b"e");
        let C_str = format!("{}", C);
        prod_str.extend_from_slice(C_str.as_bytes());

        String::from_utf8(prod_str).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 4, "24e0")]
    #[case(2, 11, "399168e2")]
    #[case(371, 375, "7219856259e3")]
    fn case(#[case] left: i32, #[case] right: i32, #[case] expected: String) {
        let actual = Solution::abbreviate_product(left, right);
        assert_eq!(actual, expected);
    }
}
