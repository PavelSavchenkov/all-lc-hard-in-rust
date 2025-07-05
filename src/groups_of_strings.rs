//! Solution for https://leetcode.com/problems/groups-of-strings
//! 2157. Groups of Strings

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

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            self.p[x] = self.get(self.p[x] as usize) as i32;
            self.p[x] as usize
        }
    }

    fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return false;
        }
        if -self.p[x] < -self.p[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[x] += self.p[y];
        self.p[y] = x as i32;
        true
    }
}

use std::collections::HashMap;

const A: usize = 26;

impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        let words = to_u8_vec(&words);
        let mut words: Vec<i32> = words
            .iter()
            .map(|w| {
                let mut mask = 0;
                for &c in w {
                    mask |= 1 << (c - b'a');
                }
                mask
            })
            .collect();
        words.sort();
        let n = words.len();

        let mut pos_of = HashMap::new();
        for (i, mask) in words.iter().enumerate() {
            pos_of.insert(mask, i);
        }

        let mut dsu = DSU::new(n);
        for (i, mask) in words.iter().enumerate() {
            let mut try_add = |nmask: i32| {
                let ii = pos_of.get(&nmask);
                if ii.is_none() {
                    return;
                }
                let ii = ii.unwrap();
                dsu.merge(i, *ii);
            };

            try_add(*mask);
            for b in 0..A {
                let nmask = mask ^ (1 << b);
                try_add(nmask);
            }
            for b0 in 0..A {
                if ((mask >> b0) & 1) == 0 {
                    continue;
                }
                for b1 in 0..A {
                    if ((mask >> b1) & 1) == 1 {
                        continue;
                    }
                    let nmask = mask ^ (1 << b0) ^ (1 << b1);
                    try_add(nmask);
                }
            }
        }

        let mut mx = 0;
        let mut cnt = 0;
        for i in 0..n {
            if dsu.p[i] < 0 {
                cnt += 1;
                mx = mx.max(-dsu.p[i]);
            }
        }

        vec![cnt as i32, mx]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["a".into(),"b".into(),"ab".into(),"cde".into()], vec![2,3])]
    #[case(vec!["a".into(),"ab".into(),"abc".into()], vec![1,3])]
    fn case(#[case] words: Vec<String>, #[case] expected: Vec<i32>) {
        let actual = Solution::group_strings(words);
        assert_eq!(actual, expected);
    }
}
