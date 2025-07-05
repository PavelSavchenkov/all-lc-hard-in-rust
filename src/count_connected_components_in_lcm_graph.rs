//! Solution for https://leetcode.com/problems/count-connected-components-in-lcm-graph
//! 3378. Count Connected Components in LCM Graph

impl Solution {
    pub fn count_components(a: Vec<i32>, threshold: i32) -> i32 {
        let threshold = threshold as usize;

        let mut dsu = DSU::new(threshold + 1);
        let mut ans = 0;
        let mut head_of = vec![0; threshold + 1];
        for &x in &a {
            let x = x as usize;
            if x > threshold {
                ans += 1;
                continue;
            }
            for l in (x..=threshold).step_by(x) {
                if head_of[l] == 0 {
                    head_of[l] = x;
                } else {
                    dsu.merge(head_of[l], x);
                }
            }
        }
        for &x in &a {
            let x = x as usize;
            if x <= threshold {
                if dsu.get(x) == x {
                    ans += 1;
                }
            }
        }
        ans as i32
    }
}

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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,4,8,3,9], 5, 4)]
    #[case(vec![2,4,8,3,9,12], 10, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] threshold: i32, #[case] expected: i32) {
        let actual = Solution::count_components(nums, threshold);
        assert_eq!(actual, expected);
    }
}
