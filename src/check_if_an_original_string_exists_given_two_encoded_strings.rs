//! Solution for https://leetcode.com/problems/check-if-an-original-string-exists-given-two-encoded-strings
//! 2060. Check if an Original String Exists Given Two Encoded Strings

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[derive(Hash, Debug)]
struct State {
    i: usize,
    j: usize,
    i_cnt: usize,
    j_cnt: usize,
}

fn go(s: &Vec<u8>, t: &Vec<u8>, state: &State, mem: &mut HashSet<u64>) {
    let h = get_hash(&state);
    // todo: one call to set for everything
    if mem.contains(&h) {
        return;
    }
    mem.insert(h);

    // eprintln!("{:#?}", state);
    assert!(state.i_cnt == 0 || state.j_cnt == 0);
    assert!(state.i_cnt < 999 && state.j_cnt < 999);

    if state.i == s.len() || state.j == t.len() {
        return;
    }

    if s[state.i].is_ascii_alphabetic() {
        assert!(state.i_cnt == 0);
        if t[state.j].is_ascii_alphabetic() {
            assert!(state.j_cnt == 0);
            if s[state.i] == t[state.j] {
                go(
                    s,
                    t,
                    &State {
                        i: state.i + 1,
                        j: state.j + 1,
                        i_cnt: 0,
                        j_cnt: 0,
                    },
                    mem,
                );
            }
            return;
        }
        let mut cur_num = 0;
        for jj in state.j..t.len() {
            if !t[jj].is_ascii_digit() {
                break;
            }
            cur_num = cur_num * 10 + (t[jj] - b'0') as usize;
            if cur_num == state.j_cnt + 1 {
                go(
                    s,
                    t,
                    &State {
                        i: state.i + 1,
                        j: jj + 1,
                        i_cnt: 0,
                        j_cnt: 0,
                    },
                    mem,
                );
            }
            if state.j_cnt + 1 < cur_num {
                go(
                    s,
                    t,
                    &State {
                        i: state.i + 1,
                        j: state.j,
                        i_cnt: 0,
                        j_cnt: state.j_cnt + 1,
                    },
                    mem,
                );
            }
        }
        return;
    }

    if t[state.j].is_ascii_alphabetic() {
        assert!(state.j_cnt == 0);
        assert!(!s[state.i].is_ascii_alphabetic());
        let mut cur_num = 0;
        for ii in state.i..s.len() {
            if !s[ii].is_ascii_digit() {
                break;
            }
            cur_num = cur_num * 10 + (s[ii] - b'0') as usize;
            if cur_num == state.i_cnt + 1 {
                go(
                    s,
                    t,
                    &State {
                        i: ii + 1,
                        j: state.j + 1,
                        i_cnt: 0,
                        j_cnt: 0,
                    },
                    mem,
                );
            }
            if state.i_cnt + 1 < cur_num {
                go(
                    s,
                    t,
                    &State {
                        i: state.i,
                        j: state.j + 1,
                        i_cnt: state.i_cnt + 1,
                        j_cnt: 0,
                    },
                    mem,
                );
            }
        }
        return;
    }

    assert!(s[state.i].is_ascii_digit() && t[state.j].is_ascii_digit());

    let mut num_i = 0;
    for ii in state.i..s.len() {
        if !s[ii].is_ascii_digit() {
            break;
        }
        num_i = num_i * 10 + (s[ii] - b'0') as usize;
        let mut num_j = 0;
        for jj in state.j..t.len() {
            if !t[jj].is_ascii_digit() {
                break;
            }
            num_j = num_j * 10 + (t[jj] - b'0') as usize;
            if state.i_cnt < num_i && state.j_cnt < num_j {
                let dist_i = num_i - state.i_cnt;
                let dist_j = num_j - state.j_cnt;
                if dist_i < dist_j {
                    go(
                        s,
                        t,
                        &State {
                            i: ii + 1,
                            i_cnt: 0,
                            j: state.j,
                            j_cnt: state.j_cnt + dist_i,
                        },
                        mem,
                    );
                } else if dist_i > dist_j {
                    go(
                        s,
                        t,
                        &State {
                            j: jj + 1,
                            j_cnt: 0,
                            i: state.i,
                            i_cnt: state.i_cnt + dist_j,
                        },
                        mem,
                    );
                } else {
                    assert!(dist_i == dist_j);
                    go(
                        s,
                        t,
                        &State {
                            i: ii + 1,
                            i_cnt: 0,
                            j: jj + 1,
                            j_cnt: 0,
                        },
                        mem,
                    );
                }
            }
        }
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

impl Solution {
    pub fn possibly_equals(s: String, t: String) -> bool {
        let s = to_u8(&s);
        let t = to_u8(&t);

        let init = State {
            i: 0,
            j: 0,
            i_cnt: 0,
            j_cnt: 0,
        };
        let mut mem = HashSet::<u64>::new();
        go(&s, &t, &init, &mut mem);

        let final_state = State {
            i: s.len(),
            j: t.len(),
            i_cnt: 0,
            j_cnt: 0,
        };
        mem.contains(&get_hash(&final_state))
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("internationalization", "i18n", true)]
    #[case("l123e", "44", true)]
    #[case("a5b", "c5b", false)]
    #[case("5ololo", "ololo5", true)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let actual = Solution::possibly_equals(s1, s2);
        assert_eq!(actual, expected);
    }
}
