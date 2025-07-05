//! Solution for https://leetcode.com/problems/russian-doll-envelopes
//! 354. Russian Doll Envelopes

struct FenwTreeMax {
    t: Vec<i64>,
}

impl FenwTreeMax {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_max(&mut self, r: usize) -> i64 {
        let mut mx = i64::MIN;
        if r == 0 {
            return mx;
        }
        let mut i = r - 1;
        loop {
            mx = mx.max(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        mx
    }
}

impl Solution {
    pub fn max_envelopes(mut a: Vec<Vec<i32>>) -> i32 {
        a.sort();
        let n = a.len();

        let mut hs = Vec::new();
        for i in 0..n {
            hs.push(a[i][1]);
        }
        hs.push(0);
        hs.sort();
        hs.dedup();

        let mut tree = FenwTreeMax::new(hs.len(), 0);

        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && a[i][0] == a[j][0] {
                j += 1;
            }
            let mut updates = Vec::new();
            for k in i..j {
                let h = a[k][1];
                let pos_h = hs.binary_search(&h).unwrap();
                let max_len = tree.get_max(pos_h);
                assert!(max_len < i64::MAX);
                updates.push((pos_h, max_len + 1));
            }
            for &(pos, len) in &updates {
                tree.relax_point(pos, len);
            }
            i = j;
        }

        tree.get_max(hs.len()) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]], 3)]
    #[case(vec![vec![1,1],vec![1,1],vec![1,1]], 1)]
    fn case(#[case] envelopes: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_envelopes(envelopes);
        assert_eq!(actual, expected);
    }
}
