//! Solution for https://leetcode.com/problems/form-largest-integer-with-digits-that-add-up-to-target
//! 1449. Form Largest Integer With Digits That Add up to Target

impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let target = target as usize;
        let cost: Vec<_> = cost.iter().map(|&c| c as usize).collect();

        let mut max_len = vec![i32::MIN; target + 1];
        max_len[0] = 0;
        for c in 0..=target {
            let cur_dp = max_len[c];
            if cur_dp < 0 {
                continue;
            }
            for dig in 1..10 {
                let nc = c + cost[dig - 1];
                if nc > target {
                    continue;
                }
                max_len[nc] = max_len[nc].max(cur_dp + 1);
            }
        }

        let mut len = max_len[target];
        let mut c = target;
        let mut ans = Vec::new();
        while len > 0 {
            for dig in (1..10).rev() {
                if c < cost[dig - 1] {
                    continue;
                }
                let prev_c = c - cost[dig - 1];
                if max_len[prev_c] + 1 == len {
                    ans.push(b'0' + dig as u8);
                    len -= 1;
                    c = prev_c;
                    break;
                }
            }
        }

        if ans.is_empty() {
            ans.push(b'0');
        }

        from_u8(&ans)
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,3,2,5,6,7,2,5,5], 9, "7772")]
    #[case(vec![7,6,5,5,5,6,8,7,8], 12, "85")]
    #[case(vec![2,4,6,2,4,6,4,4,4], 5, "0")]
    fn case(#[case] cost: Vec<i32>, #[case] target: i32, #[case] expected: String) {
        let actual = Solution::largest_number(cost, target);
        assert_eq!(actual, expected);
    }
}
