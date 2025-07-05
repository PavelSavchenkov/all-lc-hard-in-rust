//! Solution for https://leetcode.com/problems/falling-squares
//! 699. Falling Squares

struct Square {
    l: u64,
    r: u64,
}

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let squares: Vec<_> = positions
            .iter()
            .map(|p| Square {
                l: p[0] as u64,
                r: p[0] as u64 + p[1] as u64 - 1,
            })
            .collect();

        let mut xs = Vec::new();
        for s in &squares {
            xs.push(s.l);
            xs.push(s.r);
        }
        xs.sort();
        xs.dedup();

        let mut h = vec![0; xs.len()];
        let mut ans = Vec::new();
        let mut pref_mx = 0;
        for s in &squares {
            let mut mx = 0;
            for i in 0..xs.len() {
                if s.l <= xs[i] && xs[i] <= s.r {
                    mx = mx.max(h[i]);
                }
            }
            mx += s.r - s.l + 1;
            pref_mx = pref_mx.max(mx as i32);
            ans.push(pref_mx);
            for i in 0..xs.len() {
                if s.l <= xs[i] && xs[i] <= s.r {
                    h[i] = mx;
                }
            }
        }

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![2,3],vec![6,1]], vec![2,5,5])]
    #[case(vec![vec![100,100],vec![200,100]], vec![100,100])]
    fn case(#[case] positions: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::falling_squares(positions);
        assert_eq!(actual, expected);
    }
}
