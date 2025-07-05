//! Solution for https://leetcode.com/problems/find-products-of-elements-of-big-array
//! 3145. Find Products of Elements of Big Array

const B: usize = 63;

fn get_cnt_powers_up_to_num(x: i64) -> Vec<i64> {
    assert!(x >= 0);
    if x == 0 {
        return vec![0; B];
    }
    if x == 1 {
        let mut res = vec![0; B];
        res[0] = 1;
        return res;
    }
    let hb = 63 - x.leading_zeros() as usize;
    assert!(hb > 0);
    let mut res = vec![0; B];
    for i in 0..hb {
        res[i] = 1 << (hb - 1);
    }
    res[hb] = x - ((1 << hb) - 1);
    let rec = get_cnt_powers_up_to_num(x - (1 << hb));
    for i in 0..hb {
        res[i] += rec[i];
    }
    res
}

fn get_cnt_powers(i: i64) -> Vec<i64> {
    let mut L = 0;
    let mut R = i + 2;
    while L + 1 != R {
        let M = (L + R) / 2;
        let powers = get_cnt_powers_up_to_num(M);
        let cnt: i64 = powers.iter().sum();
        if cnt <= i + 1 {
            L = M;
        } else {
            R = M;
        }
    }
    let mut powers = get_cnt_powers_up_to_num(L);
    let mut cnt: i64 = powers.iter().sum();
    let mut x = L + 1;
    while cnt < i + 1 {
        assert!(x > 0);
        let lb = x.trailing_zeros() as usize;
        powers[lb] += 1;
        x ^= 1 << lb;
        cnt += 1;
    }
    powers
}

fn pow_mod(mut a: i64, mut power: u64, modulo: i64) -> i64 {
    let mut res = 1;
    while power > 0 {
        if power % 2 == 1 {
            res = (res * a) % modulo;
        }
        a = (a * a) % modulo;
        power /= 2;
    }
    res % modulo
}

fn solve(from: i64, to: i64, modulo: i32) -> i32 {
    let mut powers = get_cnt_powers(to);
    if from > 0 {
        let sub = get_cnt_powers(from - 1);
        for i in 0..B {
            powers[i] -= sub[i];
        }
    }

    let mut power = 0;
    for i in 0..B {
        power += powers[i] * i as i64;
    }
    pow_mod(2, power as u64, modulo as i64) as i32
}

impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        let mut ans = Vec::new();
        for q in &queries {
            let from = q[0];
            let to = q[1];
            let modulo = q[2] as i32;
            ans.push(solve(from, to, modulo));
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
    #[case(vec![vec![1,3,7]], vec![4])]
    #[case(vec![vec![2,5,3],vec![7,7,4]], vec![2,2])]
    fn case(#[case] queries: Vec<Vec<i64>>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_products_of_elements(queries);
        assert_eq!(actual, expected);
    }
}
