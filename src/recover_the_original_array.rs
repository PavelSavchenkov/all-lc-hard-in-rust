//! Solution for https://leetcode.com/problems/recover-the-original-array
//! 2122. Recover the Original Array

fn try_split(a: &Vec<i32>, d: i32) -> Option<Vec<i32>> {
    let n = a.len();
    let mut left = Vec::new();
    let mut used = vec![false; n];
    let mut j = 0;
    for i in 0..n {
        if used[i] {
            continue;
        }
        j = j.max(i + 1);
        while j < a.len() && (a[j] < a[i] + d || used[j]) {
            j += 1;
        }
        if j == a.len() || a[j] != a[i] + d {
            return None;
        }
        used[j] = true;
        left.push(a[i]);
    }

    Some(left)
}

impl Solution {
    pub fn recover_array(mut a: Vec<i32>) -> Vec<i32> {
        a.sort();

        for j in 0..a.len() - 1 {
            let d = a[a.len() - 1] - a[j];
            if d % 2 == 1 {
                continue;
            }
            let left = try_split(&a, d);
            if left.is_none() {
                continue;
            }
            let left = left.unwrap();
            return left.iter().map(|&x| x + d / 2).collect();
        }

        panic!("Answer must exist");
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,10,6,4,8,12], vec![3,7,11])]
    #[case(vec![1,1,3,3], vec![2,2])]
    #[case(vec![5,435], vec![220])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::recover_array(nums);
        assert_eq!(actual, expected);
    }
}
