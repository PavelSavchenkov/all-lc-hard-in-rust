//! Solution for https://leetcode.com/problems/least-operators-to-express-number
//! 964. Least Operators to Express Number

use std::collections::HashMap;

fn solve(target: u32, x: u32, mem: &mut HashMap<(u32, u32), u32>) -> u32 {
    if target == 0 {
        return 0;
    }
    let key = (target, x);
    let mem_value = mem.get(&key);
    if !mem_value.is_none() {
        return *mem_value.unwrap();
    }

    let mut pw = 0;
    let mut x_pw = 1;
    while target % x_pw == 0 && x_pw <= (target + x - 1) / x {
        pw += 1;
        x_pw *= x;
    }
    if target % x_pw != 0 {
        assert!(x_pw % x == 0);
        x_pw /= x;
        pw -= 1;
    }
    if target == x_pw {
        let ans = if pw == 0 { 1 } else { pw - 1 };
        mem.insert(key, ans);
        return ans;
    }

    let last = target % (x_pw * x);
    assert!(last != 0);
    let one = if pw == 0 { 1 } else { pw - 1 };

    // eprintln!("pw = {}, x_pw = {}, last = {}, target = {}", pw, x_pw, last, target);

    let pref_items = last / x_pw;
    assert!(pref_items < x);
    assert!(pref_items > 0);
    let mut pref_ans = one * pref_items + pref_items + solve(target - last, x, mem);
    if target == last {
        pref_ans -= 1;
    }

    let suff_items = x - pref_items;
    let mut suff_ans = one * suff_items + suff_items;
    if suff_ans < pref_ans {
        suff_ans += solve(target + x_pw * x - last, x, mem);
    } else {
        suff_ans = u32::MAX;
    }

    let ans = pref_ans.min(suff_ans);
    mem.insert(key, ans);
    ans
}

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let x = x as u32;
        let target = target as u32;
        let mut mem = HashMap::new();
        solve(target, x, &mut mem) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 19, 5)]
    #[case(5, 501, 8)]
    #[case(100, 100000000, 3)]
    fn case(#[case] x: i32, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::least_ops_express_target(x, target);
        assert_eq!(actual, expected);
    }
}
