//! Solution for https://leetcode.com/problems/split-the-array-to-make-coprime-products
//! 2584. Split the Array to Make Coprime Products

impl Solution {
    pub fn find_valid_split(a: Vec<i32>) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let M = *a.iter().max().unwrap();

        let mut min_pos = vec![n; M + 1];
        let mut max_pos = vec![0; M + 1];
        for i in 0..n {
            let x = a[i];
            min_pos[x] = min_pos[x].min(i);
            max_pos[x] = max_pos[x].max(i);
        }

        let mut buf = vec![0; n + 1];
        for d in 2..=M {
            let mut l = n;
            let mut r = 0;
            for x in (d..=M).step_by(d) {
                let ll = min_pos[x];
                let rr = max_pos[x];
                l = l.min(ll);
                r = r.max(rr);
            }
            if l < r {
                buf[l] += 1;
                buf[r] -= 1;
            }
        }

        let mut sum = 0;
        for i in 0..n - 1 {
            sum += buf[i];
            if sum == 0 {
                return i as i32;
            }
        }

        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,7,8,15,3,5], 2)]
    #[case(vec![4,7,15,8,3,5], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_valid_split(nums);
        assert_eq!(actual, expected);
    }
}
