//! Solution for https://leetcode.com/problems/subsequences-with-a-unique-middle-mode-i
//! 3395. Subsequences with a Unique Middle Mode I

impl Solution {
    pub fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
        let mut vals = nums.clone();
        vals.sort();
        vals.dedup();

        let mut a: Vec<usize> = nums
            .iter()
            .map(|x| vals.binary_search(&x).unwrap())
            .collect();
        let n = a.len();

        let C2 = |cnt: usize| -> i64 {
            if cnt <= 1 {
                return 0;
            }
            let cnt = cnt as i64;
            cnt * (cnt - 1) / 2
        };

        let mut ans = 0;
        for it in 0..2 {
            let mut cnt_left = vec![0; vals.len()];
            let mut cnt_right = vec![0 as usize; vals.len()];
            for i in 0..n {
                cnt_right[a[i]] += 1;
            }
            let mut sum_sq_cnt_left = 0;
            for i in 0..n {
                cnt_right[a[i]] -= 1;
                let C = a[i];

                // A. more C to the right of mid that to the left of mid
                // cnt[mid] == 2; d, b, c, a, c
                for j in i + 1..n {
                    let A = a[j];
                    if A == C {
                        continue;
                    }
                    let left = i - cnt_left[C] - cnt_left[A];
                    if left >= 2 {
                        let mut coef = left as i64 * left as i64 - sum_sq_cnt_left;
                        coef += cnt_left[A] as i64 * cnt_left[A] as i64;
                        coef += cnt_left[C] as i64 * cnt_left[C] as i64;
                        coef /= 2;
                        coef *= cnt_right[C] as i64;
                        ans += coef;
                    }
                }

                // cnt[mid] == 3;
                if cnt_right[C] >= 2 {
                    // !=c, !=c, c, c, c
                    let left = i - cnt_left[C];
                    if left >= 2 {
                        let coef = C2(left) * C2(cnt_right[C]);
                        ans += coef;
                    }
                }

                // cnt[mid] == 4; !=c, c, c, c, c
                if cnt_left[C] >= 1 && cnt_right[C] >= 2 {
                    let mut coef = C2(cnt_right[C]);
                    let left = i - cnt_left[C];
                    coef *= left as i64;
                    coef *= cnt_left[C] as i64;
                    ans += coef;
                }

                // B. the same number of C on both sides
                if it == 0 {
                    // cnt[mid] == 3; !=c, c, c, c, !=c
                    if cnt_left[C] >= 1 && cnt_right[C] >= 1 {
                        let left = i - cnt_left[C];
                        let right = (n - i - 1) - cnt_right[C];
                        let mut coef = left as i64 * cnt_left[C] as i64;
                        coef *= right as i64 * cnt_right[C] as i64;
                        ans += coef;
                    }
                    // cnt[mid] == 5
                    if cnt_left[C] >= 2 && cnt_right[C] >= 2 {
                        let mut coef = 1;
                        for cnt in [cnt_left[C], cnt_right[C]] {
                            coef *= C2(cnt);
                        }
                        ans += coef;
                    }
                }

                // eprintln!("it={}, i={}, ans={}", it, i, ans.val);

                sum_sq_cnt_left -= cnt_left[a[i]] as i64 * cnt_left[a[i]] as i64;
                cnt_left[a[i]] += 1;
                sum_sq_cnt_left += cnt_left[a[i]] as i64 * cnt_left[a[i]] as i64;
            }

            a.reverse();
        }

        (ans % 1_000_000_007) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,1,1,1,1], 6)]
    #[case(vec![1,2,2,3,3,4], 4)]
    #[case(vec![0,1,2,3,4,5,6,7,8], 0)]
    #[case(vec![1,1,1,1,0,-1,1], 15)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::subsequences_with_middle_mode(nums);
        assert_eq!(actual, expected);
    }
}
