//! Solution for https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests
//! 1601. Maximum Number of Achievable Transfer Requests

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = requests.len();

        let mut ans = 0;
        for mask in 1..((1 << m) as usize) {
            let mut deg = vec![0; n];
            for i in 0..m {
                if ((mask >> i) & 1) == 1 {
                    let from = requests[i][0] as usize;
                    let to = requests[i][1] as usize;
                    deg[from] += 1;
                    deg[to] -= 1;
                }
            }
            let achiveable = deg.iter().filter(|&&d| d != 0).next().is_none();
            if achiveable {
                ans = ans.max(mask.count_ones());
            }
        }
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![0,1],vec![1,0],vec![0,1],vec![1,2],vec![2,0],vec![3,4]], 5)]
    #[case(3, vec![vec![0,0],vec![1,2],vec![2,1]], 3)]
    #[case(4, vec![vec![0,3],vec![3,1],vec![1,2],vec![2,0]], 4)]
    fn case(#[case] n: i32, #[case] requests: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximum_requests(n, requests);
        assert_eq!(actual, expected);
    }
}
