//! Solution for https://leetcode.com/problems/count-array-pairs-divisible-by-k
//! 2183. Count Array Pairs Divisible by K

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut a: Vec<_> = nums.iter().map(|&x| x as usize).collect();
        let k = k as usize;
        let n = a.len();

        for x in &mut a {
            *x %= k;
        }

        let is_div_k: Vec<_> = (0..=k).map(|i| i > 0 && k % i == 0).collect();
        let mut divs = vec![Vec::new(); k + 1];
        for d in 1..=k {
            if !is_div_k[d] {
                continue;
            }
            for num in (d..=k).step_by(d) {
                divs[num].push(d);
            }
            divs[0].push(d);
        }

        let mut ans: u64 = 0;
        let mut cnt = vec![0; k + 1];
        for i in 0..n {
            let have_d = gcd(k, a[i]);
            let need_d = k / have_d;
            ans += cnt[need_d] as u64;

            for &d in &divs[a[i]] {
                cnt[d] += 1;
            }
        }

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
    #[case(vec![1,2,3,4,5], 2, 7)]
    #[case(vec![1,2,3,4], 5, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_pairs(nums, k);
        assert_eq!(actual, expected);
    }
}
