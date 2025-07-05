//! Solution for https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations
//! 3003. Maximize the Number of Partitions After Operations

const A: usize = 26;

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let s = to_u8(&s);
        let n = s.len();
        let k = k as usize;

        if k == A {
            return 1;
        }

        let mut next = vec![vec![n; A]; n + 1];
        for i in (0..n).rev() {
            for c in 0..A {
                next[i][c] = next[i + 1][c];
            }
            let c = (s[i] - b'a') as usize;
            next[i][c] = i;
        }

        let mut suff = vec![0; n + 1];
        for i in (0..n).rev() {
            let mut by_next: Vec<_> = (0..A).collect();
            by_next.sort_by_key(|&c| next[i][c]);
            let next_i = next[i][by_next[k]];
            suff[i] = suff[next_i] + 1;
        }

        let ans_if = |mut start: usize, i: usize, c: usize| -> usize {
            let mut ans = 0;
            while start <= i {
                let mut cur_next = next[start].clone();
                cur_next[c] = cur_next[c].min(i);
                let old_c = (s[i] - b'a') as usize;
                if c == old_c {
                    return suff[start];
                }
                if cur_next[old_c] == i {
                    cur_next[old_c] = next[i + 1][old_c];
                }
                let mut by_next: Vec<_> = (0..A).collect();
                by_next.sort_by_key(|&cc| cur_next[cc]);
                ans += 1;
                start = cur_next[by_next[k]];
            }
            ans + suff[start]
        };

        let mut ans = suff[0];
        let mut i = 0;
        let mut pref = 0;
        while i < n {
            let mut by_next: Vec<_> = (0..A).collect();
            by_next.sort_by_key(|&c| next[i][c]);

            let next_i = next[i][by_next[k]];
            for j in i..next_i {
                for c in 0..A {
                    let cur = pref + ans_if(i, j, c);
                    ans = ans.max(cur);
                }
            }

            i = next_i;
            pref += 1;
        }
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
    #[case("accca", 2, 3)]
    #[case("aabaab", 3, 1)]
    #[case("xxyz", 1, 4)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_partitions_after_operations(s, k);
        assert_eq!(actual, expected);
    }
}
