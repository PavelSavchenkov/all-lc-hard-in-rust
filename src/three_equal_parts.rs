//! Solution for https://leetcode.com/problems/three-equal-parts
//! 927. Three Equal Parts

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();

        let mut p1 = Vec::new();
        for i in 0..n {
            if arr[i] == 1 {
                p1.push(i);
            }
        }
        if p1.len() % 3 != 0 {
            return vec![-1, -1];
        }

        let cnt = p1.len() / 3;
        if cnt == 0 {
            return vec![0, n as i32 - 1];
        }

        let a = p1[cnt] - p1[cnt - 1] - 1;
        let b = p1[2 * cnt] - p1[2 * cnt - 1] - 1;
        let c = n - p1[3 * cnt - 1] - 1;
        if a < c || b < c {
            return vec![-1, -1];
        }

        let pattern = &arr[p1[0]..p1[cnt - 1] + 1];
        if pattern != &arr[p1[cnt]..p1[2 * cnt - 1] + 1] {
            return vec![-1, -1];
        }
        if pattern != &arr[p1[2 * cnt]..p1[3 * cnt - 1] + 1] {
            return vec![-1, -1];
        }

        let i = p1[cnt - 1] + c;
        let j = p1[2 * cnt - 1] + c + 1;
        let ans = vec![i as i32, j as i32];
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
    #[case(vec![1,0,1,0,1], vec![0,3])]
    #[case(vec![1,1,0,1,1], vec![-1,-1])]
    #[case(vec![1,1,0,0,1], vec![0,2])]
    fn case(#[case] arr: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::three_equal_parts(arr);
        assert_eq!(actual, expected);
    }
}
