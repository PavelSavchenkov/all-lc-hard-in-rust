//! Solution for https://leetcode.com/problems/maximum-total-reward-using-operations-ii
//! 3181. Maximum Total Reward Using Operations II

const W: usize = 64;

fn set_bits_seg(mask: &mut u64, l: usize, len: usize, bits: u64) {
    let mut r = l + len;
    r = r.min(W);
    if l >= r {
        return;
    }
    let mask_pref = if r == W { u64::MAX } else { (1 << r) - 1 };
    let mask_seg = mask_pref - ((1 << l) - 1);
    *mask &= !mask_seg;
    *mask ^= mask_seg & (bits << l);
}

#[derive(Clone)]
struct BitSet {
    b: Vec<u64>,
    nbits: usize,
}

impl BitSet {
    fn new(nbits: usize) -> Self {
        Self {
            b: vec![0; (nbits + W - 1) / W],
            nbits,
        }
    }

    fn test(&self, i: usize) -> bool {
        ((self.b[i / W] >> (i % W)) & 1) == 1
    }

    fn set_seg(&mut self, l: usize, mut len: usize, bits: u64) {
        if l >= self.nbits {
            return;
        }
        assert!(len <= W);
        len = len.min(self.nbits - l);

        let i = l / W;
        set_bits_seg(&mut self.b[i], l % W, len, bits);
        if (l + len - 1) / W > i {
            let pref = W - l % W;
            set_bits_seg(&mut self.b[i + 1], 0, len - pref, bits >> pref);
        }
    }

    fn shift_right(&mut self, len: usize) {
        for i in (0..self.b.len()).rev() {
            let l = i * W + len;
            self.set_seg(l, W, self.b[i]);
        }
        for l in (0..len).step_by(W) {
            let len_seg = W.min(len - l);
            self.set_seg(l, len_seg, 0);
        }
    }

    fn or_with(&mut self, other: &Self) {
        assert!(self.nbits == other.nbits);
        for i in 0..self.b.len() {
            self.b[i] |= other.b[i];
        }
    }

    fn and_with(&mut self, other: &Self) {
        assert!(self.nbits == other.nbits);
        for i in 0..self.b.len() {
            self.b[i] &= other.b[i];
        }
    }

    fn count_ones(&self) -> usize {
        let mut ans = 0;
        for &b in &self.b {
            ans += b.count_ones() as usize;
        }
        ans
    }
}

impl Solution {
    pub fn max_total_reward(mut ws: Vec<i32>) -> i32 {
        ws.sort();
        ws.dedup();
        let n = ws.len();

        let N = 2 * *ws.last().unwrap() as usize;
        let mut dp = BitSet::new(N);
        dp.set_seg(0, 1, 1);

        let mut pref_ones = BitSet::new(N);
        let mut l_one = 0;

        for &w in &ws {
            let w = w as usize;
            while l_one < 2 * w {
                pref_ones.set_seg(l_one, 1, 1);
                l_one += 1;
            }
            assert!(pref_ones.count_ones() == 2 * w);

            let mut row = dp.clone();
            row.shift_right(w);
            row.and_with(&pref_ones);

            dp.or_with(&row);

            assert!(dp.test(w));
        }

        for w in (0..N).rev() {
            if dp.test(w) {
                return w as i32;
            }
        }
        unreachable!()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,3,3], 4)]
    #[case(vec![1,6,4,3,2], 11)]
    #[case(vec![46], 46)]
    fn case(#[case] reward_values: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_total_reward(reward_values);
        assert_eq!(actual, expected);
    }
}
