//! Solution for https://leetcode.com/problems/similar-string-groups
//! 839. Similar String Groups

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

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let strs = to_u8_vec(&strs);
        let n = strs.len();
        let len = strs[0].len();

        let mut dsu = DSU::new(n);
        for i in 0..n {
            for j in 0..i {
                let cnt_diff = strs[i]
                    .iter()
                    .zip(strs[j].iter())
                    .filter(|(&a, &b)| a != b)
                    .take(3)
                    .count();
                if cnt_diff <= 2 {
                    dsu.merge(i, j);
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            if i == dsu.get(i) {
                ans += 1;
            }
        }

        ans
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
    #[case(vec!["tars".into(),"rats".into(),"arts".into(),"star".into()], 2)]
    #[case(vec!["omv".into(),"ovm".into()], 1)]
    fn case(#[case] strs: Vec<String>, #[case] expected: i32) {
        let actual = Solution::num_similar_groups(strs);
        assert_eq!(actual, expected);
    }
}
