//! Solution for https://leetcode.com/problems/largest-component-size-by-common-factor
//! 952. Largest Component Size by Common Factor

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

    fn comp_size(&mut self, x: usize) -> usize {
        let root = self.get(x);
        (-self.p[root]) as usize
    }
}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let n = a.len();
        let M = *a.iter().max().unwrap();

        let mut have = vec![false; M + 1];
        for &x in &a {
            have[x] = true;
        }

        let mut dsu = DSU::new(M + 1);
        for d in 2..=M {
            let mut prev = 0;
            for x in (d..=M).step_by(d) {
                if have[x] {
                    if prev > 0 {
                        dsu.merge(prev, x);
                    }
                    prev = x;
                }
            }
        }

        let mut size = vec![0; M + 1];
        for &x in &a {
            size[dsu.get(x)] += 1;
        }
        *size.iter().max().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,6,15,35], 4)]
    #[case(vec![20,50,9,63], 2)]
    #[case(vec![2,3,6,7,4,12,21,39], 8)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::largest_component_size(nums);
        assert_eq!(actual, expected);
    }
}
