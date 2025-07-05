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