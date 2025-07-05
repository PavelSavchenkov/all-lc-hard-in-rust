//! Solution for https://leetcode.com/problems/make-the-xor-of-all-segments-equal-to-zero
//! 1787. Make the XOR of All Segments Equal to Zero

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn min_changes(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as usize).collect();
        let k = k as usize;

        let mut who = vec![Vec::new(); k];
        for i in 0..k {
            let mut vals = Vec::new();
            for j in (i..n).step_by(k) {
                vals.push(a[j]);
            }
            vals.sort();
            let mut prev = vals[0];
            let bucket = &mut who[i];
            bucket.push((prev, 1 as usize));
            for j in 1..vals.len() {
                if vals[j] == prev {
                    bucket.last_mut().unwrap().1 += 1;
                } else {
                    prev = vals[j];
                    bucket.push((vals[j], 1));
                }
            }
        }

        let mut max_hb = 0;
        for &x in &a {
            if x != 0 {
                let hb = std::mem::size_of::<usize>() * 8 - 1 - x.leading_zeros() as usize;
                max_hb = max_hb.max(hb);
            }
        }
        let M = 1 << (max_hb + 1);
        let mut dp = vec![usize::MAX; M];
        dp[0] = 0;
        for i in 0..k {
            let mut ndp = vec![usize::MAX; M];
            let all = who[i].iter().fold(0, |acc, e| acc + e.1);
            for prev_xor in 0..M {
                let prev_dp = dp[prev_xor];
                if prev_dp == usize::MAX {
                    continue;
                }
                for &(val, cnt) in &who[i] {
                    let new_xor = prev_xor ^ val;
                    let add_dp = all - cnt;
                    remin(&mut ndp[new_xor], prev_dp + add_dp);
                }
            }
            let min_prev = *dp.iter().min().unwrap();
            for xor in 0..M {
                remin(&mut ndp[xor], min_prev + all);
            }
            dp = ndp;
        }
        dp[0] as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,0,3,0], 1, 3)]
    #[case(vec![3,4,5,2,1,7,3,4,7], 3, 3)]
    #[case(vec![1,2,4,1,2,5,1,2,6], 3, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_changes(nums, k);
        assert_eq!(actual, expected);
    }
}
