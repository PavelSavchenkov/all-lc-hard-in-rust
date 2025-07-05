//! Solution for https://leetcode.com/problems/minimum-white-tiles-after-covering-with-carpets
//! 2209. Minimum White Tiles After Covering With Carpets

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let a: Vec<_> = floor
            .as_bytes()
            .iter()
            .map(|&b| (b - b'0') as u32)
            .collect();
        let num_carpets = num_carpets as usize;
        let carpet_len = carpet_len as usize;
        let n = a.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i];
        }

        // max deleted ones
        let mut dp = vec![0; n + 1];
        for it in 0..num_carpets {
            let mut ndp = dp.clone();
            for i in 0..n {
                ndp[i + 1] = ndp[i + 1].max(ndp[i]);
                let r = n.min(i + carpet_len);
                let deleted = pref[r] - pref[i];
                ndp[r] = ndp[r].max(dp[i] + deleted);
            }
            dp = ndp;
        }

        let ans = pref[n] - dp[n];
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
    #[case("10110101", 2, 2, 2)]
    #[case("11111", 2, 3, 0)]
    fn case(
        #[case] floor: String,
        #[case] num_carpets: i32,
        #[case] carpet_len: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_white_tiles(floor, num_carpets, carpet_len);
        assert_eq!(actual, expected);
    }
}
