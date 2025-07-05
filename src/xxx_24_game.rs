//! Solution for https://leetcode.com/problems/24-game
//! 679. 24 Game

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Fraction {
    p: i32,
    q: i32,
}

impl Fraction {
    fn new(x: i32) -> Self {
        Self { p: x, q: 1 }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            p: self.p * other.p,
            q: self.q * other.q,
        }
        .normalize()
    }

    fn div(&self, other: &Self) -> Self {
        Self {
            p: self.p * other.q,
            q: self.q * other.p,
        }
        .normalize()
    }

    fn add(&self, other: &Self) -> Self {
        let Q = lcm(self.q, other.q);
        Self {
            p: self.p * (Q / self.q) + other.p * (Q / other.q),
            q: Q,
        }
        .normalize()
    }

    fn sub(&self, other: &Self) -> Self {
        let Q = lcm(self.q, other.q);
        Self {
            p: self.p * (Q / self.q) - other.p * (Q / other.q),
            q: Q,
        }
        .normalize()
    }

    fn normalize(&self) -> Self {
        let g = gcd(self.p.abs(), self.q.abs());
        let mut res = Self {
            p: self.p / g,
            q: self.q / g,
        };
        if res.q < 0 {
            res.p = -res.p;
            res.q = -res.q;
        }
        if res.p == 0 {
            res.q = 1;
        }
        res
    }
}

fn rec(nums: Vec<Fraction>) -> bool {
    if nums.len() == 1 {
        let x = nums[0];
        if x.p == 24 && x.q == 1 {
            return true;
        }
        return false;
    }
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let a = nums[i];
            let b = nums[j];
            let mut vals = Vec::new();
            vals.push(Fraction::mul(&a, &b));
            if b.p != 0 {
                vals.push(Fraction::div(&a, &b));
            }
            vals.push(Fraction::add(&a, &b));
            vals.push(Fraction::sub(&a, &b));
            for val in vals {
                let mut new_nums = nums.clone();
                new_nums[i] = val;
                new_nums.remove(j);
                if rec(new_nums) {
                    return true;
                }
            }
        }
    }
    false
}

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let nums: Vec<_> = cards.iter().map(|&x| Fraction::new(x)).collect();
        rec(nums)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,1,8,7], true)]
    #[case(vec![1,2,1,2], false)]
    fn case(#[case] cards: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::judge_point24(cards);
        assert_eq!(actual, expected);
    }
}
