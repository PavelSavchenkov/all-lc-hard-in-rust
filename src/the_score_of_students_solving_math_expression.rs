//! Solution for https://leetcode.com/problems/the-score-of-students-solving-math-expression
//! 2019. The Score of Students Solving Math Expression

use std::char::MAX;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const MUL: i32 = -1;
const ADD: i32 = -2;
const MAX_TOKENS: usize = 31;
const V: usize = 1000;

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn go(
    tokens: [i32; MAX_TOKENS],
    cnt_tokens: usize,
    was: &mut HashSet<u64>,
    vals: &mut HashSet<i32>,
    correct: &mut i32,
    is_correct: bool,
) {
    assert!(cnt_tokens > 0);
    if cnt_tokens == 1 {
        let val = tokens[0];
        assert!(val >= 0);
        vals.insert(val);
        if is_correct {
            *correct = val;
        }
        return;
    }
    assert!(cnt_tokens % 2 == 1);
    let h = get_hash(&(tokens, is_correct));
    if was.contains(&h) {
        return;
    }

    let have_muls = tokens[0..cnt_tokens].iter().filter(|&&t| t == MUL).count() > 0;
    for i in (1..cnt_tokens).step_by(2) {
        assert!(i % 2 == 1);
        assert!(i + 1 < cnt_tokens);

        let l = tokens[i - 1];
        let r = tokens[i + 1];
        let op = tokens[i];
        let token = if op == MUL { l * r } else { l + r };

        let new_cnt_tokens = cnt_tokens - 2;
        let mut new_tokens = [0; MAX_TOKENS];
        new_tokens[0..i - 1].copy_from_slice(&tokens[0..i - 1]);
        new_tokens[i - 1] = token;
        new_tokens[i..new_cnt_tokens].copy_from_slice(&tokens[i + 2..cnt_tokens]);

        // eprintln!("tokens={tokens:#?}, new_tokens={new_tokens:#?}");

        let new_is_correct = is_correct && (op == MUL || !have_muls);
        go(
            new_tokens,
            new_cnt_tokens,
            was,
            vals,
            correct,
            new_is_correct,
        );
    }

    was.insert(h);
}

impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let tokens: Vec<i32> = s
            .as_bytes()
            .iter()
            .map(|c| match c {
                b'*' => MUL,
                b'+' => ADD,
                _ => (c - b'0') as i32,
            })
            .collect();

        let n = tokens.len();
        let mut dp = [[[false; V + 1]; MAX_TOKENS + 1]; MAX_TOKENS + 1];
        for len in (1..=n).step_by(2) {
            for l in 0..=n - len {
                if l % 2 != 0 {
                    continue;
                }
                let r = l + len;
                if len == 1 {
                    dp[l][r][tokens[l] as usize] = true;
                    continue;
                }
                for i in (l + 1..r).step_by(2) {
                    let op = tokens[i];
                    assert!(op < 0);
                    for vl in 0..=V {
                        if !dp[l][i][vl] {
                            continue;
                        }
                        for vr in 0..=V {
                            if !dp[i + 1][r][vr] {
                                continue;
                            }
                            let v = if op == MUL { vl * vr } else { vl + vr };
                            if v > V {
                                break;
                            }
                            dp[l][r][v] = true;
                        }
                    }
                }
            }
        }

        let mut vals = HashSet::<i32>::new();
        for v in 0..=V {
            if dp[0][n][v] {
                vals.insert(v as i32);
            }
        }

        let correct = {
            let mut st = Vec::new();
            for &t in &tokens {
                if !st.is_empty() && st[st.len() - 1] == MUL {
                    st.pop();
                    let l = st.pop().unwrap();
                    st.push(l * t);
                } else {
                    st.push(t);
                }
            }
            let mut sum = 0;
            for i in (0..st.len()).step_by(2) {
                sum += st[i];
            }
            sum
        };

        // let mut was = HashSet::<u64>::new();
        // let mut vals = HashSet::<i32>::new();
        // let mut correct = -1;
        // let mut tokens_array = [0; MAX_TOKENS];
        // tokens_array[..tokens.len()].copy_from_slice(&tokens[..]);
        // go(
        //     tokens_array,
        //     tokens.len(),
        //     &mut was,
        //     &mut vals,
        //     &mut correct,
        //     true,
        // );
        // assert!(correct != -1);
        // eprintln!("vals.len()={}, was.len()={}", vals.len(), was.len());

        let mut ans = 0;
        for student in answers {
            if student == correct {
                ans += 5;
            } else if vals.contains(&student) {
                ans += 2;
            }
        }

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
    #[case("7+3*1*2", vec![20,13,42], 7)]
    #[case("3+5*2", vec![13,0,10,13,13,16,16], 19)]
    #[case("6+0*1", vec![12,9,6,4,8,6], 10)]
    #[case("0*0+0+0+0*0", vec![0,0,2,1,1000], 10)]
    #[case("4*8*4+8*4*8*2+4*6+4*8+8*6+2*2", vec![1000], 0)]
    fn case(#[case] s: String, #[case] answers: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::score_of_students(s, answers);
        assert_eq!(actual, expected);
    }
}
