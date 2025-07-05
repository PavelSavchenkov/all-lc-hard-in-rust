//! Solution for https://leetcode.com/problems/create-sorted-array-through-instructions
//! 1649. Create Sorted Array through Instructions

struct FenwTreeSum {
    t: Vec<i64>,
}

impl FenwTreeSum {
    fn new(n: usize) -> Self {
        Self { t: vec![0; n] }
    }

    fn add_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] += val;
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_sum_pref(&self, r: usize) -> i64 {
        if r == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut i = r - 1;
        loop {
            sum += self.t[i];
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        sum
    }

    fn get_sum(&self, l: usize, r: usize) -> i64 {
        if l >= r {
            return 0;
        }
        self.get_sum_pref(r) - self.get_sum_pref(l)
    }
}

impl Solution {
    pub fn create_sorted_array(a: Vec<i32>) -> i32 {
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let N = *a.iter().max().unwrap();

        let mut tree = FenwTreeSum::new(N + 1);
        let mut ans = 0;
        for &x in &a {
            let up = tree.get_sum(x + 1, N + 1);
            let down = tree.get_sum(0, x);
            let func = up.min(down);
            ans += func as i64;

            tree.add_point(x, 1);
        }
        (ans % 1_000_000_007) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,5,6,2], 1)]
    #[case(vec![1,2,3,6,5,4], 3)]
    #[case(vec![1,3,3,3,2,4,2,1,2], 4)]
    fn case(#[case] instructions: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::create_sorted_array(instructions);
        assert_eq!(actual, expected);
    }
}
