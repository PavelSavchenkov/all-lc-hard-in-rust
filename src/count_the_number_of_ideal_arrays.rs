//! Solution for https://leetcode.com/problems/count-the-number-of-ideal-arrays
//! 2338. Count the Number of Ideal Arrays

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let n = n as usize;
        let max_value = max_value as usize;

        let mut cnt_chains = vec![Vec::<Num>::new(); max_value + 1];
        for x in 1..=max_value {
            cnt_chains[x].push(Num::new(0));
            cnt_chains[x].push(Num::new(1));
        }

        let mut len = 2;
        loop {
            let mut was_update = false;
            for x in 1..=max_value {
                while cnt_chains[x].len() <= len {
                    cnt_chains[x].push(Num::new(0));
                }
                for up in (2 * x..=max_value).step_by(x) {
                    if cnt_chains[up].len() >= len {
                        let up_dp = cnt_chains[up][len - 1];
                        if up_dp.val == 0 {
                            continue;
                        }
                        cnt_chains[x][len] += up_dp;
                        was_update = true;
                    }
                }
            }
            if !was_update {
                break;
            }
            len += 1;
        }
        let max_chain_len = len;
        let mut cnt_chains_accumulated = vec![Num::new(0); max_chain_len + 1];
        for x in 1..=max_value {
            for len in 1..cnt_chains[x].len() {
                cnt_chains_accumulated[len] += cnt_chains[x][len];
            }
        }

        let mut dp = vec![vec![Num::new(0); max_chain_len + 1]; n + 1];
        dp[0][1] = Num::new(1);
        for cnt_pref in 0..n {
            for cnt_distinct in 1..=max_chain_len {
                let cur_dp = dp[cnt_pref][cnt_distinct];
                dp[cnt_pref + 1][cnt_distinct] += cur_dp;
                if cnt_distinct < max_chain_len && cnt_pref > 0 {
                    dp[cnt_pref + 1][cnt_distinct + 1] += cur_dp;
                }
            }
        }

        // eprintln!("cnt_chains_accumulated = {:#?}", cnt_chains_accumulated);
        // eprintln!("dp = {:#?}", dp);

        let mut ans = Num::new(0);
        for chain_len in 1..=max_chain_len {
            ans += dp[n][chain_len] * cnt_chains_accumulated[chain_len];
        }

        ans.val as i32
    }
}

use std::cmp::Eq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

const MOD: u32 = 1_000_000_007;
type Num = NumMod<MOD>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct NumMod<const M: u32> {
    val: u32,
}

impl<const M: u32> NumMod<M> {
    // doesn't work
    const static_assert: () = {
        assert!(M < (1 << 31));
    };

    fn new(x: u32) -> Self {
        NumMod { val: x % M }
    }

    // x and y are already normalized
    fn safe_add_mod(mut x: u32, y: u32) -> u32 {
        x += y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_sub_mod(mut x: u32, y: u32) -> u32 {
        x += M - y;
        if x >= M {
            x -= M;
        }
        x
    }

    fn safe_mul_mod(x: u32, y: u32) -> u32 {
        ((x as u64 * y as u64) % M as u64) as u32
    }
}

impl<const M: u32> Add for NumMod<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_add_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> AddAssign for NumMod<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const M: u32> Sub for NumMod<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_sub_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> SubAssign for NumMod<M> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const M: u32> Mul for NumMod<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            val: Self::safe_mul_mod(self.val, other.val),
        }
    }
}

impl<const M: u32> MulAssign for NumMod<M> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 5, 10)]
    #[case(5, 3, 11)]
    fn case(#[case] n: i32, #[case] max_value: i32, #[case] expected: i32) {
        let actual = Solution::ideal_arrays(n, max_value);
        assert_eq!(actual, expected);
    }
}
