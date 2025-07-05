//! Solution for https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference
//! 2035. Partition Array Into Two Arrays to Minimize Sum Difference

fn go(a: &[i32], i: usize, taken: usize, n: usize, s: i32, ans: &mut Vec<Vec<i32>>) {
    ans[taken].push(s);
    if i == n {
        return;
    }
    go(a, i + 1, taken + 1, n, s + a[i], ans);
    go(a, i + 1, taken, n, s, ans);
}

fn precalc(a: &[i32], n: usize) -> Vec<Vec<i32>> {
    let mut ans = vec![Vec::<i32>::new(); n + 1];
    go(a, 0, 0, n, 0, &mut ans);
    for c in 0..=n {
        ans[c].sort();
        ans[c].dedup();
    }
    ans
}

impl Solution {
    pub fn minimum_difference(a: Vec<i32>) -> i32 {
        assert!(a.len() % 2 == 0);
        let n = a.len() / 2;
        let S = a.iter().sum();

        let mut ans = i32::MAX;
        let left = precalc(&a[..n], n);
        let right = precalc(&a[n..], n);
        for c in 0..n {
            for s in &left[c] {
                let other = &right[n - c];
                let mut i = other.partition_point(|&x| (s + x) * 2 < S);
                for it in 0..2 {
                    if i < other.len() {
                        let total_s = s + other[i];
                        ans = ans.min((S - total_s * 2).abs());
                    }
                    if i == 0 {
                        break;
                    }
                    i -= 1;
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
    #[case(vec![3,9,7,3], 2)]
    #[case(vec![-36,36], 72)]
    #[case(vec![2,-1,0,4,-2,-9], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_difference(nums);
        assert_eq!(actual, expected);
    }
}
