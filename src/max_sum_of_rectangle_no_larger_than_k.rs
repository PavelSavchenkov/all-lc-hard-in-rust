//! Solution for https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k
//! 363. Max Sum of Rectangle No Larger Than K

impl Solution {
    pub fn max_sum_submatrix(a: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = a.len();
        let m = a[0].len();

        let mut mx = -100 * n as i32 * m as i32;
        for i0 in 0..n {
            let mut arr = vec![0; m];
            for i1 in i0..n {
                for j in 0..m {
                    arr[j] += a[i1][j];
                }
                for j in 0..m {
                    let mut sum = 0;
                    for j1 in j..m {
                        sum += arr[j1];
                        if sum <= k && sum > mx {
                            mx = sum;
                        }
                    }
                }
            }
        }

        mx
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,0,1],vec![0,-2,3]], 2, 2)]
    #[case(vec![vec![2,2,-1]], 3, 3)]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_sum_submatrix(matrix, k);
        assert_eq!(actual, expected);
    }
}
