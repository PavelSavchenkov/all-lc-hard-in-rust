//! Solution for https://leetcode.com/problems/find-array-given-subset-sums
//! 1982. Find Array Given Subset Sums

impl Solution {
    fn split(a: &Vec<i32>, val: u32) -> Option<(Vec<i32>, Vec<i32>)> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut used = vec![false; a.len()];

        let mut j = 0;
        for i in 0..a.len() {
            if used[i] {
                continue;
            }
            j = j.max(i + 1);
            while j < a.len() && (used[j] || a[j] - a[i] < val as i32) {
                j += 1;
            }
            if j == a.len() || a[j] - a[i] != val as i32 {
                return None;
            }
            left.push(a[i]);
            right.push(a[j]);
            used[i] = true;
            used[j] = true;
        }
        Some((left, right))
    }

    fn solve(n: usize, sums: Vec<i32>) -> Option<Vec<i32>> {
        assert!(sums.len() == (1 << n));

        if n == 0 {
            if sums[0] == 0 {
                return Some(Vec::<i32>::new());
            } else {
                return None;
            }
        }

        let elem = if sums[sums.len() - 1] > 0 {
            sums[sums.len() - 1] - sums[sums.len() - 2]
        } else {
            sums[1] - sums[0]
        } as u32;

        let sums_split = Self::split(&sums, elem);
        if sums_split.is_none() {
            return None;
        }
        let (left, right) = sums_split.unwrap();

        let left_res = Self::solve(n - 1, left);
        if !left_res.is_none() {
            let mut left_res = left_res.unwrap();
            left_res.push(elem as i32);
            return Some(left_res);
        }

        let right_res = Self::solve(n - 1, right);
        if !right_res.is_none() {
            let mut right_res = right_res.unwrap();
            right_res.push(-(elem as i32));
            return Some(right_res);
        }

        None
    }

    pub fn recover_array(n: i32, mut sums: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        sums.sort();

        Self::solve(n, sums).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![-3,-2,-1,0,0,1,2,3], vec![1,2,-3])]
    #[case(2, vec![0,0,0,0], vec![0,0])]
    #[case(4, vec![0,0,5,5,4,-1,4,9,9,-1,4,3,4,8,3,8], vec![0,-1,4,5])]
    fn case(#[case] n: i32, #[case] sums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::recover_array(n, sums);
        assert_eq!(actual, expected);
    }
}
