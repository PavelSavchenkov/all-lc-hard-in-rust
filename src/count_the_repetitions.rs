//! Solution for https://leetcode.com/problems/count-the-repetitions
//! 466. Count The Repetitions

const A: usize = 26;
const LOG: usize = 30;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = to_u8(&s1);
        let s2 = to_u8(&s2);
        let n1 = n1 as usize;
        let n2 = n2 as usize;

        // (i, j) --> (nxt[i][s2[j]], (j + 1) % s2.len()), w == number of chars used from s1
        let N = s1.len() * s2.len();
        let mut to = vec![vec![N; LOG]; N];
        let mut weight = vec![vec![0 as u64; LOG]; N];
        for i in 0..s1.len() {
            for j in 0..s2.len() {
                let v = i * s2.len() + j;
                let ch = s2[j];
                for len in 1..=s1.len() {
                    let ii = (i + len - 1) % s1.len();
                    if s1[ii] == s2[j] {
                        let next_i = (ii + 1) % s1.len();
                        let u = next_i * s2.len() + (j + 1) % s2.len();
                        to[v][0] = u;
                        weight[v][0] = len as u64;
                        break;
                    }
                }
            }
        }

        for l in 1..LOG {
            for v in 0..N {
                let v_mid = to[v][l - 1];
                if v_mid != N {
                    to[v][l] = to[to[v][l - 1]][l - 1];
                    weight[v][l] = weight[v][l - 1] + weight[to[v][l - 1]][l - 1];
                } else {
                    to[v][l] = N;
                    weight[v][l] = u64::MAX;
                }
            }
        }

        let max_weight = s1.len() as u64 * n1 as u64;
        let mut cnt_edges = 0;
        let mut accumulated_weight = 0;
        let mut v = 0;
        for pw2 in (0..LOG).rev() {
            if weight[v][pw2] < u64::MAX && accumulated_weight + weight[v][pw2] <= max_weight {
                accumulated_weight += weight[v][pw2];
                v = to[v][pw2];
                cnt_edges += 1 << pw2;
            }
        }

        let ans = (cnt_edges) / (s2.len() * n2);
        ans as i32
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
    #[case("acb", 4, "ab", 2, 2)]
    #[case("acb", 1, "acb", 1, 1)]
    fn case(
        #[case] s1: String,
        #[case] n1: i32,
        #[case] s2: String,
        #[case] n2: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::get_max_repetitions(s1, n1, s2, n2);
        assert_eq!(actual, expected);
    }
}
