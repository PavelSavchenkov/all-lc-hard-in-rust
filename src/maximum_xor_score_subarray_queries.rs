//! Solution for https://leetcode.com/problems/maximum-xor-score-subarray-queries
//! 3277. Maximum XOR Score Subarray Queries

impl Solution {
    pub fn maximum_subarray_xor(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = a.len();

        let mut ans = vec![vec![0; n + 1]; n + 1];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                if len == 1 {
                    ans[l][r] = a[l];
                } else {
                    ans[l][r] = ans[l][r - 1] ^ ans[l + 1][r];
                }
            }
        }

        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                if l > 0 {
                    ans[l - 1][r] = ans[l - 1][r].max(ans[l][r]);
                }
                if r + 1 < n {
                    ans[l][r + 1] = ans[l][r + 1].max(ans[l][r]);
                }
            }
        }

        let mut answers = Vec::new();
        for q in &queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            answers.push(ans[l][r]);
        }

        answers

        // let n = 50;
        // let mut binom = vec![vec![0; n + 1]; n + 1];
        // binom[0][0] = 1;
        // for i in 1..=n {
        //     binom[i][0] = 1;
        //     for j in 1..=i {
        //         binom[i][j] = binom[i - 1][j] ^ binom[i - 1][j - 1];
        //     }
        // }

        // for i in 0..=n {
        //     eprint!("n={}: ", i);
        //     for j in 0..=i {
        //         eprint!("{}", binom[i][j]);
        //     }
        //     eprintln!();
        // }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,8,4,32,16,1], vec![vec![0,2],vec![1,4],vec![0,5]], vec![12,60,60])]
    #[case(vec![0,7,3,2,8,5,1], vec![vec![0,3],vec![1,5],vec![2,4],vec![2,6],vec![5,6]], vec![7,14,11,14,5])]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::maximum_subarray_xor(nums, queries);
        assert_eq!(actual, expected);
    }
}
