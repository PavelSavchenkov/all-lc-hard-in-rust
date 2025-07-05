//! Solution for https://leetcode.com/problems/subarrays-with-k-different-integers
//! 992. Subarrays with K Different Integers

fn calc_min_length(a: &Vec<usize>, k: usize) -> Vec<usize> {
    let update = |x: usize, change: i32, cnt: &mut Vec<i32>, cnt_diff: &mut usize| {
        if cnt[x] != 0 {
            *cnt_diff -= 1;
        }
        cnt[x] += change;
        if cnt[x] != 0 {
            *cnt_diff += 1;
        }
    };

    let n = a.len();
    let M = *a.iter().max().unwrap();
    let mut cnt = vec![0; M + 1];
    let mut cnt_diff = 0;
    let mut r = 0; // first not taken
    let mut len = vec![0; n];
    for i in 0..n {
        assert!(i <= r);
        while r < n && cnt_diff < k {
            update(a[r], 1, &mut cnt, &mut cnt_diff);
            r += 1;
        }
        if cnt_diff < k {
            r = n + 1;
        }
        len[i] = r - i;
        update(a[i], -1, &mut cnt, &mut cnt_diff);
    }

    len
}

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let k = k as usize;

        let len_k = calc_min_length(&a, k);
        let len_bigger = calc_min_length(&a, k + 1);

        let mut ans = 0;
        for i in 0..n {
            let min = len_k[i];
            let max = len_bigger[i];
            assert!(min <= max);
            ans += max - min;
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
    #[case(vec![1,2,1,2,3], 2, 7)]
    #[case(vec![1,2,1,3,4], 3, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::subarrays_with_k_distinct(nums, k);
        assert_eq!(actual, expected);
    }
}
