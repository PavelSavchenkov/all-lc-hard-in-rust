//! Solution for https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers
//! 1320. Minimum Distance to Type a Word Using Two Fingers

const A: usize = 26;

impl Solution {
    pub fn minimum_distance(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut dist = vec![vec![0; A]; A];
        for a in 0..A {
            for b in 0..A {
                let xa = a as i32 / 6;
                let ya = a as i32 % 6;
                let xb = b as i32 / 6;
                let yb = b as i32 % 6;
                dist[a][b] = ((xa - xb).abs() + (ya - yb).abs()) as usize;
            }
        }

        let mut dp = vec![0; A];
        for i in 1..n {
            let mut ndp = vec![usize::MAX; A];
            for c1 in 0..A {
                let cur_dp = dp[c1];
                let c2 = (s[i - 1] - b'A') as usize;

                let c_dest = (s[i] - b'A') as usize;
                // c1 --> c_dest
                ndp[c2] = ndp[c2].min(cur_dp + dist[c1][c_dest]);
                // c2 --> c_dest
                ndp[c1] = ndp[c1].min(cur_dp + dist[c2][c_dest]);
            }
            dp = ndp;
        }
        let ans = *dp.iter().min().unwrap();
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
    #[case("CAKE", 3)]
    #[case("HAPPY", 6)]
    fn case(#[case] word: String, #[case] expected: i32) {
        let actual = Solution::minimum_distance(word);
        assert_eq!(actual, expected);
    }
}
