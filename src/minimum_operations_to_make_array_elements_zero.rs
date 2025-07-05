//! Solution for https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero
//! 3495. Minimum Operations to Make Array Elements Zero

fn calc(l: i64, r: i64) -> Vec<i64> {
    // ans[i] -- how many numbers from [l; r] such that steps[x] == i
    let mut ans = Vec::new();
    let mut steps = 0;
    let mut L = 0;
    let mut R = 0;
    while L <= r {
        let cnt = (r.min(R) - l.max(L) + 1).max(0);
        ans.push(cnt);
        steps += 1;
        L = R + 1;
        R = R * 4 + 3;
    }
    ans
}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;
        for q in &queries {
            let l = q[0];
            let r = q[1];

            let cnts = calc(l as i64, r as i64);
            let mut sum = 0;
            let mut mx = 0;
            for i in 0..cnts.len() {
                if cnts[i] > 0 {
                    sum += i as i64 * cnts[i];
                    mx = i as i64;
                }
            }
            assert!(mx > 0);
            assert!(sum >= mx);
            let sum_rest = sum - mx;
            let cur;
            if mx <= sum_rest {
                cur = sum / 2 + sum % 2;
            } else {
                cur = mx;
            }
            ans += cur;
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
    #[case(vec![vec![1,2],vec![2,4]], 3)]
    #[case(vec![vec![2,6]], 4)]
    fn case(#[case] queries: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::min_operations(queries);
        assert_eq!(actual, expected);
    }
}
