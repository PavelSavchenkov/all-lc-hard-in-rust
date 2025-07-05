//! Solution for https://leetcode.com/problems/maximize-active-section-with-trade-ii
//! 3501. Maximize Active Section with Trade II

struct SegmTreeMax {
    t: Vec<usize>,
    sz: usize,
}

impl SegmTreeMax {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            t: vec![usize::MIN; sz * 2],
            sz,
        }
    }

    fn update(&mut self, pos: usize, val: usize) {
        let mut v = self.sz + pos;
        self.t[v] = val;
        v /= 2;
        while v > 0 {
            self.t[v] = self.t[v * 2].max(self.t[v * 2 + 1]);
            v /= 2;
        }
    }

    fn get_max(&self, mut l: usize, mut r: usize) -> usize {
        r = r.min(self.sz - 1);
        let mut ans = usize::MIN;
        l += self.sz;
        r += self.sz;
        while l <= r {
            ans = ans.max(self.t[l]);
            ans = ans.max(self.t[r]);
            l = (l + 1) / 2;
            r = (r - 1) / 2;
        }
        ans
    }
}

#[derive(Debug)]
struct Seg {
    l: usize,
    r: usize,
}

impl Seg {
    fn len(&self) -> usize {
        if self.l < self.r {
            self.r - self.l
        } else {
            0
        }
    }

    fn inter(&self, l: usize, r: usize) -> Self {
        Self {
            l: self.l.max(l),
            r: self.r.min(r),
        }
    }
}

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = to_u8(&s);
        let n = s.len();

        let mut segs = Vec::new();
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && s[i] == s[j] {
                j += 1;
            }
            if s[i] == b'0' {
                segs.push(Seg { l: i, r: j });
            }
            i = j;
        }

        let mut tree = SegmTreeMax::new(segs.len());
        for i in 1..segs.len() {
            tree.update(i - 1, segs[i - 1].len() + segs[i].len());
        }

        let cnt1 = s.iter().filter(|&&c| c == b'1').count();

        let mut ans = Vec::new();
        for q in &queries {
            let l = q[0] as usize;
            let r = q[1] as usize;

            let mut cur = 0;
            let left_seg = segs.partition_point(|s| s.r <= l);
            let mut right_seg = segs.partition_point(|s| s.l <= r);
            if right_seg > 0 {
                right_seg -= 1;
            }
            if left_seg + 3 <= right_seg {
                cur = cur.max(tree.get_max(left_seg + 1, right_seg - 2));
            }
            // eprintln!("segs = {:#?}, left_seg={}, right_seg={}", segs, left_seg, right_seg);
            let mut cands = Vec::new();
            if left_seg < segs.len() {
                cands.push(left_seg);
            }
            if right_seg > 0 {
                cands.push(right_seg - 1);
            }
            for &i in &cands {
                if i + 1 < segs.len() {
                    let one = segs[i].inter(l, r + 1);
                    let two = segs[i + 1].inter(l, r + 1);
                    if one.len() > 0 && two.len() > 0 {
                        cur = cur.max(one.len() + two.len());
                    }
                }
            }
            cur += cnt1;

            ans.push(cur as i32);
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
    #[case("01", vec![vec![0,1]], vec![1])]
    #[case("0100", vec![vec![0,3],vec![0,2],vec![1,3],vec![2,3]], vec![4,3,1,1])]
    #[case("1000100", vec![vec![1,5],vec![0,6],vec![0,4]], vec![6,7,2])]
    #[case("01010", vec![vec![0,3],vec![1,4],vec![1,3]], vec![4,4,2])]
    #[case("0101110001101", vec![vec![0,7],vec![0,12]], vec![10, 11])]
    fn case(#[case] s: String, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_active_sections_after_trade(s, queries);
        assert_eq!(actual, expected);
    }
}
