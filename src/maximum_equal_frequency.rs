//! Solution for https://leetcode.com/problems/maximum-equal-frequency
//! 1224. Maximum Equal Frequency

impl Solution {
    pub fn max_equal_freq(a: Vec<i32>) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let M = *a.iter().max().unwrap();

        let mut cnt = vec![0 as usize; M + 1];
        let mut cnt_cnt = vec![0 as usize; n + 1];
        let mut cnt_distinct = 0;
        let mut ans = 0;
        for i in 0..n {
            let x = a[i];

            if cnt[x] == 0 {
                cnt_distinct += 1;
            } else {
                cnt_cnt[cnt[x]] -= 1;
            }

            cnt[x] += 1;
            cnt_cnt[cnt[x]] += 1;

            let mut ok = false;

            // deleted number occurs one time
            {
                // every remaining number also occurs one time
                if cnt_cnt[1] == cnt_distinct {
                    ok = true;
                }

                // x is deleted number
                if cnt[x] == 1 {
                    if i == 0 {
                        ok = true;
                    } else {
                        let y = a[i - 1];
                        if cnt[y] != 1 && cnt_cnt[cnt[y]] == cnt_distinct - 1 {
                            ok = true;
                        }
                    }
                }

                // x is not deleted number
                if cnt[x] > 1 && cnt_cnt[1] == 1 && cnt_cnt[cnt[x]] == cnt_distinct - 1 {
                    ok = true;
                }
            }
            // deleted number becomes equal all others after deletion
            {
                // x is deleted number
                if cnt_cnt[cnt[x] - 1] == cnt_distinct - 1 {
                    assert!(cnt_cnt[cnt[x]] == 1);
                    ok = true;
                }
                // x is not deleted number
                if cnt_cnt[cnt[x]] == cnt_distinct - 1 && cnt_cnt[cnt[x] + 1] == 1 {
                    ok = true;
                }
            }

            if ok {
                ans = ans.max(i + 1);
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
    #[case(vec![2,2,1,1,5,3,3,5], 7)]
    #[case(vec![1,1,1,2,2,2,3,3,3,4,4,4,5], 13)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_equal_freq(nums);
        assert_eq!(actual, expected);
    }
}
