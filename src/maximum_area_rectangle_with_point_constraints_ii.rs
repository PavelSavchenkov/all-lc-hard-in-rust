//! Solution for https://leetcode.com/problems/maximum-area-rectangle-with-point-constraints-ii
//! 3382. Maximum Area Rectangle With Point Constraints II

struct SegmTreeMin {
    t: Vec<i32>,
    sz: usize,
}

impl SegmTreeMin {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            t: vec![i32::MAX; sz * 2],
            sz,
        }
    }

    // [l; r]
    fn get_min(&self, mut l: usize, mut r: usize) -> i32 {
        let mut ans = i32::MAX;
        l += self.sz;
        r += self.sz;
        while l <= r {
            ans = ans.min(self.t[l]);
            ans = ans.min(self.t[r]);
            l = (l + 1) / 2;
            r = (r - 1) / 2;
        }
        ans
    }

    fn update(&mut self, pos: usize, val: i32) {
        let mut v = self.sz + pos;
        self.t[v] = val;
        v /= 2;
        while v > 0 {
            self.t[v] = self.t[v * 2].min(self.t[v * 2 + 1]);
            v /= 2;
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Solution {
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        let n = x_coord.len();

        let mut ps = Vec::new();
        for i in 0..n {
            ps.push(Point {
                x: x_coord[i],
                y: y_coord[i],
            });
        }
        ps.sort_by_key(|p| (-p.y, p.x));

        let mut xs = x_coord;
        xs.sort();
        xs.dedup();

        let mut tree = SegmTreeMin::new(xs.len());
        let mut ans = -1;
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && ps[i].y == ps[j].y {
                j += 1;
            }
            let y0 = ps[i].y;

            for k in i + 1..j {
                let x0 = ps[k - 1].x;
                let x1 = ps[k].x;
                assert!(x0 < x1);

                let pos_x0 = xs.binary_search(&x0).unwrap();
                let pos_x1 = xs.binary_search(&x1).unwrap();
                let y1 = tree.get_min(pos_x0, pos_x0);
                assert!(y1 > y0);
                if y1 == i32::MAX {
                    continue;
                }
                if tree.get_min(pos_x1, pos_x1) != y1 {
                    continue;
                }
                if pos_x0 + 1 < pos_x1 {
                    if tree.get_min(pos_x0 + 1, pos_x1 - 1) <= y1 {
                        continue;
                    }
                }
                let area = (x1 - x0) as i64 * (y1 - y0) as i64;
                ans = ans.max(area);
            }

            for k in i..j {
                let pos_x = xs.binary_search(&ps[k].x).unwrap();
                tree.update(pos_x, y0);
            }

            i = j;
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
    #[case(vec![1,1,3,3], vec![1,3,1,3], 4)]
    #[case(vec![1,1,3,3,2], vec![1,3,1,3,2], -1)]
    #[case(vec![1,1,3,3,1,3], vec![1,3,1,3,2,2], 2)]
    fn case(#[case] x_coord: Vec<i32>, #[case] y_coord: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::max_rectangle_area(x_coord, y_coord);
        assert_eq!(actual, expected);
    }
}
