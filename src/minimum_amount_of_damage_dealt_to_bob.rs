//! Solution for https://leetcode.com/problems/minimum-amount-of-damage-dealt-to-bob
//! 3273. Minimum Amount of Damage Dealt to Bob

impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        let n = damage.len();

        let mut c = vec![0; n];
        for i in 0..n {
            c[i] = (health[i] + power - 1) / power;
        }

        let mut ord: Vec<_> = (0..n).collect();
        ord.sort_by(|&i, &j| {
            let l = damage[i] * c[j];
            let r = damage[j] * c[i];
            r.cmp(&l)
        });

        let mut ans = 0;
        let mut sum_c = 0;
        for &i in &ord {
            sum_c += c[i] as i64;
            ans += damage[i] as i64 * sum_c;
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
    #[case(4, vec![1,2,3,4], vec![4,5,6,8], 39)]
    #[case(1, vec![1,1,1,1], vec![1,2,3,4], 20)]
    #[case(8, vec![40], vec![59], 320)]
    fn case(
        #[case] power: i32,
        #[case] damage: Vec<i32>,
        #[case] health: Vec<i32>,
        #[case] expected: i64,
    ) {
        let actual = Solution::min_damage(power, damage, health);
        assert_eq!(actual, expected);
    }
}
