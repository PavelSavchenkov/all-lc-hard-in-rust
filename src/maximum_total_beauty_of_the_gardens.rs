//! Solution for https://leetcode.com/problems/maximum-total-beauty-of-the-gardens
//! 2234. Maximum Total Beauty of the Gardens

impl Solution {
    pub fn maximum_beauty(a: Vec<i32>, to_add: i64, target: i32, full: i32, partial: i32) -> i64 {
        let mut a: Vec<_> = a.iter().map(|&x| x as u64).collect();
        let to_add = to_add as u64;
        let target = target as u64;
        let full = full as u64;
        let partial = partial as u64;

        for x in &mut a {
            *x = target.min(*x);
        }
        a.push(target);
        a.sort();
        let n = a.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i];
        }

        let mut ans = 0;
        for i in (0..n).rev() {
            // [i, n) are completed
            let spent_to_complete = target * (n - i) as u64 - (pref[n] - pref[i]);
            if spent_to_complete > to_add {
                break;
            }

            let rem_add = to_add - spent_to_complete;

            let mut L = 0;
            let mut R = target;
            while L + 1 != R {
                let M = (L + R) / 2;
                // can we do M = min_incomplete?
                let mut j = a.partition_point(|&x| x < M);
                j = j.min(i);
                let spent_to_reach_M = M * j as u64 - pref[j];
                if spent_to_reach_M <= rem_add {
                    L = M
                } else {
                    R = M
                }
            }
            let min_incomplete = if i == 0 || a[0] >= target { 0 } else { L };

            let cur_ans = (n - i) as u64 * full + min_incomplete * partial;

            // eprintln!(
            //     "i = {}, min_incomplete = {}, cur_ans = {}",
            //     i, min_incomplete, cur_ans
            // );
            ans = ans.max(cur_ans);
        }

        ans -= full; // because we added fake "target" garden
        ans as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,1,1], 7, 6, 12, 1, 14)]
    #[case(vec![2,4,5,3], 10, 5, 2, 6, 30)]
    fn case(
        #[case] flowers: Vec<i32>,
        #[case] new_flowers: i64,
        #[case] target: i32,
        #[case] full: i32,
        #[case] partial: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::maximum_beauty(flowers, new_flowers, target, full, partial);
        assert_eq!(actual, expected);
    }
}
