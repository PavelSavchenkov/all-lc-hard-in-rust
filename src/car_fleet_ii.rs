//! Solution for https://leetcode.com/problems/car-fleet-ii
//! 1776. Car Fleet II

struct Car {
    pos: i32,
    speed: i32,
}

impl Car {
    fn new(v: &Vec<i32>) -> Self {
        Self {
            pos: v[0],
            speed: v[1],
        }
    }
}

use std::{cmp::Ordering, collections::BTreeSet};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 && b != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    a + b
}

#[derive(Eq, PartialEq)]
struct Fraction {
    p: i32,
    q: i32,
}

impl Fraction {
    fn new(p: i32, q: i32) -> Self {
        Self { p, q }.norm()
    }

    fn norm(&self) -> Self {
        let mut p = self.p;
        let mut q = self.q;
        if q < 0 {
            q = -q;
            p = -p;
        }
        let g = gcd(p.abs(), q.abs());
        p /= g;
        q /= g;
        Self { p, q }
    }

    fn as_float(&self) -> f64 {
        self.p as f64 / self.q as f64
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.p as i64 * other.q as i64).cmp(&(other.p as i64 * self.q as i64))
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Event {
    t: Fraction,
    i: usize, // by inc or by dec?
}

impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let cars: Vec<_> = cars.iter().map(|v| Car::new(&v)).collect();
        let n = cars.len();

        let mut next = vec![0; n];
        let mut prev = vec![0; n];
        for i in 0..n {
            next[i] = (i + 1) % n;
            prev[i] = (i + n - 1) % n;
        }

        let mut es = BTreeSet::<Event>::new();
        for i in 0..n - 1 {
            // todo: a helper function to add new event
            let dx = cars[i + 1].pos - cars[i].pos;
            let dv = cars[i].speed - cars[i + 1].speed;
            if dv > 0 {
                es.insert(Event {
                    t: Fraction::new(dx, dv),
                    i,
                });
            }
        }

        let mut ans = vec![-1.0; n];
        while !es.is_empty() {
            let e = es.pop_first().unwrap();
            if ans[e.i] > -0.5 {
                continue;
            }

            ans[e.i] = e.t.as_float();

            let left = prev[e.i];
            let right = next[e.i];
            if left == e.i {
                assert!(right == e.i);
                break;
            }

            next[left] = right;
            prev[right] = left;

            let dx = cars[right].pos - cars[left].pos;
            if dx > 0 {
                let dv = cars[left].speed - cars[right].speed;
                if dv > 0 {
                    es.insert(Event {
                        t: Fraction::new(dx, dv),
                        i: left,
                    });
                }
            }
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
    #[case(vec![vec![1,2],vec![2,1],vec![4,3],vec![7,2]], vec![1.00000,-1.00000,3.00000,-1.00000])]
    #[case(vec![vec![3,4],vec![5,4],vec![6,3],vec![9,1]], vec![2.00000,1.00000,1.50000,-1.00000])]
    fn case(#[case] cars: Vec<Vec<i32>>, #[case] expected: Vec<f64>) {
        let actual = Solution::get_collision_times(cars);
        assert_eq!(actual, expected);
    }
}
