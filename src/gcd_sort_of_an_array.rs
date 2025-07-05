//! Solution for https://leetcode.com/problems/gcd-sort-of-an-array
//! 1998. GCD Sort of an Array

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
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let nums: Vec<_> = nums.iter().map(|&x| x as usize).collect();
        let n = nums.len();

        let M = *nums.iter().max().unwrap();
        let mut have = vec![false; M + 1];
        for &num in &nums {
            have[num as usize] = true;
        }

        let mut dsu = DSU::new(M + 1);
        for g in 2..=M {
            let mut root = 0;
            for x in (g..=M).step_by(g) {
                if root == 0 && have[x] {
                    root = x;
                }
                if have[x] {
                    dsu.merge(root, x);
                }
            }
        }

        let mut nums_with_root = vec![Vec::new(); M + 1];
        for &num in &nums {
            let c = dsu.get(num);
            nums_with_root[c].push(num);
        }
        for c in 0..=M {
            nums_with_root[c].sort();
            nums_with_root[c].reverse();
        }

        let mut mx1 = 0;
        let mut comp1 = 0;
        let mut mx2 = 0;
        let mut comp2 = 0;
        for i in 0..n {
            let c = dsu.get(nums[i]);
            let num = nums_with_root[c].pop().unwrap();

            if num < mx1 && c != comp1 {
                return false;
            }
            if num < mx2 && c != comp2 {
                return false;
            }
            if c == comp1 {
                mx1 = mx1.max(num);
                continue;
            }
            if c == comp2 {
                mx2 = mx2.max(num);
                if mx2 > mx1 {
                    std::mem::swap(&mut mx1, &mut mx2);
                    std::mem::swap(&mut comp1, &mut comp2);
                }
                continue;
            }
            if num > mx1 {
                mx2 = mx1;
                comp2 = comp1;
                mx1 = num;
                comp1 = c;
            } else if num > mx2 {
                mx2 = num;
                comp2 = c;
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
    #[case(vec![7,21,3], true)]
    #[case(vec![5,2,6,2], false)]
    #[case(vec![10,5,9,3,15], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::gcd_sort(nums);
        assert_eq!(actual, expected);
    }
}
