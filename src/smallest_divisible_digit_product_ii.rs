//! Solution for https://leetcode.com/problems/smallest-divisible-digit-product-ii
//! 3348. Smallest Divisible Digit Product II

fn factor(mut x: i64) -> Option<Vec<i32>> {
    let mut cnt = vec![0; 10];
    if x <= 1 {
        return Some(cnt);
    }
    for d in 2..10 {
        while x % d == 0 {
            cnt[d as usize] += 1;
            x /= d;
        }
    }
    if x != 1 {
        return None;
    }
    Some(cnt)
}

fn norm(s: &Vec<u8>) -> Vec<u8> {
    let mut s = s.clone();
    s.reverse();
    while *s.last().unwrap() == b'0' {
        s.pop();
        assert!(!s.is_empty());
    }
    s.reverse();
    s
}

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        let mut num = to_u8(&num);

        let need_degs = factor(t);
        if need_degs.is_none() {
            return "-1".to_string();
        }
        let need_degs = need_degs.unwrap();

        let can_complete = |pref_degs: &Vec<i32>, suff_len: usize| -> bool {
            for cnt6 in 0..=2 {
                let mut need_len = cnt6;
                let need2 = need_degs[2] - pref_degs[2] - cnt6;
                if need2 > 0 {
                    need_len += (need2 + 2) / 3;
                }
                let need3 = need_degs[3] - pref_degs[3] - cnt6;
                if need3 > 0 {
                    need_len += (need3 + 1) / 2;
                }
                let need5 = need_degs[5] - pref_degs[5];
                if need5 > 0 {
                    need_len += need5;
                }
                let need7 = need_degs[7] - pref_degs[7];
                if need7 > 0 {
                    need_len += need7;
                }
                if need_len as usize <= suff_len {
                    return true;
                }
            }
            false
        };

        let sum_degs = |degs1: &Vec<i32>, degs2: &Vec<i32>, coef2: i32| -> Vec<i32> {
            assert!(degs1.len() == degs2.len());
            let mut res = vec![0; degs1.len()];
            for i in 0..res.len() {
                res[i] = degs1[i] + coef2 * degs2[i];
            }
            res
        };

        let PREF_ZEROS = 60;
        num.reverse();
        for it in 0..PREF_ZEROS {
            num.push(b'0');
        }
        num.reverse();
        let n = num.len();

        let mut pref_degs = vec![0; 10];
        let mut first_zero = n;
        for i in PREF_ZEROS..n {
            let d = (num[i] - b'0') as i64;
            if d == 0 {
                first_zero = first_zero.min(i);
                continue;
            }
            let degs = factor(d).unwrap();
            pref_degs = sum_degs(&pref_degs, &degs, 1);
        }

        if first_zero == n && can_complete(&pref_degs, 0) {
            return from_u8(&norm(&num));
        }

        for i in (0..n).rev() {
            let cur_d = (num[i] - b'0') as i64;
            let cur_degs = factor(cur_d).unwrap();
            pref_degs = sum_degs(&pref_degs, &cur_degs, -1);
            for new_d in cur_d + 1..10 {
                let new_degs = factor(new_d).unwrap();
                let cand_degs = sum_degs(&pref_degs, &new_degs, 1);
                if i <= first_zero && can_complete(&cand_degs, n - i - 1) {
                    // eprintln!("build from i={}", i);
                    num[i] = new_d as u8 + b'0';
                    pref_degs = cand_degs;
                    for j in i + 1..n {
                        let mut set_d = 0;
                        for d in 1..10 {
                            let degs = factor(d).unwrap();
                            let cand_degs = sum_degs(&pref_degs, &degs, 1);
                            if can_complete(&cand_degs, n - j - 1) {
                                set_d = d;
                                pref_degs = cand_degs;
                                break;
                            }
                        }
                        assert!(set_d > 0);
                        num[j] = b'0' + set_d as u8;
                    }
                    return from_u8(&norm(&num));
                }
            }
        }

        return "-1".to_string();
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
    #[case("1234", 256, "1488")]
    #[case("12355", 50, "12355")]
    #[case("11111", 26, "-1")]
    fn case(#[case] num: String, #[case] t: i64, #[case] expected: String) {
        let actual = Solution::smallest_number(num, t);
        assert_eq!(actual, expected);
    }
}
