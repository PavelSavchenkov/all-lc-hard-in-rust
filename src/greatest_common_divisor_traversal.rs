//! Solution for https://leetcode.com/problems/greatest-common-divisor-traversal
//! 2709. Greatest Common Divisor Traversal

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.p[y] = x as i32;
        true
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(a: Vec<i32>) -> bool {
        let n = a.len();

        if n == 1 {
            return true;
        }

        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let M = *a.iter().max().unwrap();

        if M == 1 {
            return false;
        }

        let mut was = vec![false; M + 1];
        for &x in &a {
            was[x] = true;
        }

        let mut dsu = DSU::new(M + 1);
        for d in 2..=M {
            let mut prev = 0;
            for x in (d..=M).step_by(d) {
                if was[x] {
                    if prev > 0 {
                        dsu.merge(prev, x);
                    }
                    prev = x;
                }
            }
        }

        for i in 0..n {
            if dsu.get(a[i]) != dsu.get(a[0]) {
                return false;
            }
        }

        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,6], true)]
    #[case(vec![3,9,5], false)]
    #[case(vec![4,3,12,8], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_traverse_all_pairs(nums);
        assert_eq!(actual, expected);
    }
}
