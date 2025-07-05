//! Solution for https://leetcode.com/problems/maximize-grid-happiness
//! 1659. Maximize Grid Happiness

fn remax(a: &mut i32, b: i32) {
    if *a < b {
        *a = b;
    }
}

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let intro_total = introverts_count as usize;
        let extro_total = extroverts_count as usize;

        let mut pow3 = vec![1; n + 1];
        for i in 1..pow3.len() {
            pow3[i] = pow3[i - 1] * 3;
        }
        let get_bit = |mut mask: usize, i: usize| -> usize {
            mask /= pow3[i];
            mask % 3
        };

        let get_score_nei = |old: usize, new: usize| -> i32 {
            if new == 0 || old == 0 {
                return 0;
            }
            if new == 1 && old == 1 {
                return -60;
            }
            if new == 2 && old == 2 {
                return 40;
            }
            return -10;
        };

        let max_mask = (3 as usize).pow(n as u32) - 1;

        let mut ans = i32::MIN;
        // col_mask (3-base), intro_left, extro_left
        let mut dp = vec![vec![vec![i32::MIN; extro_total + 1]; intro_total + 1]; max_mask + 1];
        dp[0][intro_total][extro_total] = 0;
        for j in 0..m {
            for i in 0..n {
                let mut ndp =
                    vec![vec![vec![i32::MIN; extro_total + 1]; intro_total + 1]; max_mask + 1];
                for intro_left in 0..=intro_total {
                    for extro_left in 0..=extro_total {
                        for mask in 0..=max_mask {
                            let cur_dp = dp[mask][intro_left][extro_left];
                            if cur_dp == i32::MIN {
                                continue;
                            }
                            for who in 0..3 {
                                let mut nmask = mask;
                                nmask -= pow3[i] * get_bit(mask, i);
                                nmask += pow3[i] * who;
                                let mut score = 0;
                                if who != 0 {
                                    if i > 0 {
                                        score += get_score_nei(get_bit(mask, i - 1), who);
                                    }
                                    score += get_score_nei(get_bit(mask, i), who);
                                    if who == 1 {
                                        score += 120;
                                    } else {
                                        score += 40;
                                    }
                                }
                                let mut new_intro_left = intro_left;
                                let mut new_extro_left = extro_left;
                                if who == 1 {
                                    if new_intro_left == 0 {
                                        continue;
                                    }
                                    new_intro_left -= 1;
                                } else if who == 2 {
                                    if new_extro_left == 0 {
                                        continue;
                                    }
                                    new_extro_left -= 1;
                                }
                                let dp_cand = cur_dp + score;
                                remax(&mut ndp[nmask][new_intro_left][new_extro_left], dp_cand);
                                if i == n - 1 && j == m - 1 {
                                    ans = ans.max(dp_cand);
                                }
                            }
                        }
                    }
                }
                dp = ndp;
            }
        }
        assert!(ans >= 0);
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
    #[case(2, 3, 1, 2, 240)]
    #[case(3, 1, 2, 1, 260)]
    #[case(2, 2, 4, 0, 240)]
    fn case(
        #[case] m: i32,
        #[case] n: i32,
        #[case] introverts_count: i32,
        #[case] extroverts_count: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);
        assert_eq!(actual, expected);
    }
}
