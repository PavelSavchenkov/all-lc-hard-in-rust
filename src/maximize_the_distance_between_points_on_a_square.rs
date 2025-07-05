//! Solution for https://leetcode.com/problems/maximize-the-distance-between-points-on-a-square
//! 3464. Maximize the Distance Between Points on a Square

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut ps: Vec<_> = points.iter().map(|v| Point { x: v[0], y: v[1] }).collect();
        let n = ps.len();

        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut up = Vec::new();
        let mut down = Vec::new();
        for p in &ps {
            if p.x == 0 && p.y < side {
                left.push(p.y);
            } else if p.y == side && p.x < side {
                up.push(p.x);
            } else if p.x == side && p.y > 0 {
                right.push(p.y);
            } else if p.y == 0 && p.x > 0 {
                down.push(p.x);
            } else {
                panic!()
            }
        }
        left.sort();
        up.sort();
        right.sort();
        right.reverse();
        down.sort();
        down.reverse();

        // 1. try d > side
        if k == 4 {
            // at most one point on each side
            // k >= 4, so exactly one on each side

            let check = |d: i32| -> bool {
                if d <= side {
                    return true;
                }
                for i in 0..left.len() {
                    let j = up.partition_point(|x| side - left[i] + x < d);
                    if j == up.len() {
                        continue;
                    }
                    let mut z = down.partition_point(|x| left[i] + x >= d);
                    if z == 0 {
                        continue;
                    }
                    z -= 1;
                    let mut l = 0;
                    let mut r = right.len();
                    let p = right.partition_point(|y| side - up[j] + side - y < d);
                    l = l.max(p);
                    let p = right.partition_point(|y| side - down[z] + y >= d);
                    r = r.min(p);
                    if l < r {
                        if (right[l] - left[i]).abs() + side >= d {
                            return true;
                        }
                        if (right[r - 1] - left[i]).abs() + side >= d {
                            return true;
                        }
                    }
                }
                false
            };

            let mut L = side as i64;
            let mut R = 2 * side as i64 + 1;
            while L + 1 != R {
                let M = (L + R) / 2;
                if check(M as i32) {
                    L = M;
                } else {
                    R = M;
                }
            }
            let L = L as i32;
            if L >= side + 1 {
                return L;
            }
        }

        // 2. d <= side ---> points on the opposite sides are okay
        ps.clear();
        let mut a = Vec::new();
        for i in 0..left.len() {
            if i + 1 < left.len() {
                a.push(left[i + 1] - left[i]);
            } else if !up.is_empty() {
                a.push(side - left[i] + up[0]);
            } else {
                a.push(2 * side + 1);
            }
            ps.push(Point { x: 0, y: left[i] });
        }
        for i in 0..up.len() {
            if i + 1 < up.len() {
                a.push(up[i + 1] - up[i]);
            } else if !right.is_empty() {
                a.push(side - up[i] + side - right[0]);
            } else {
                a.push(2 * side + 1);
            }
            ps.push(Point { x: up[i], y: side });
        }
        for i in 0..right.len() {
            if i + 1 < right.len() {
                a.push(right[i] - right[i + 1]);
            } else if !down.is_empty() {
                a.push(right[i] + side - down[0]);
            } else {
                a.push(2 * side + 1);
            }
            ps.push(Point {
                x: side,
                y: right[i],
            });
        }
        for i in 0..down.len() {
            if i + 1 < down.len() {
                a.push(down[i] - down[i + 1]);
            } else if !left.is_empty() {
                a.push(down[i] + left[0]);
            } else {
                a.push(2 * side + 1);
            }
            ps.push(Point { x: down[i], y: 0 });
        }
        assert!(a.len() == n);

        let check = |d: i32| -> bool {
            if d > side {
                return false;
            }
            if d <= 1 {
                return true;
            }

            let mut next = vec![0; n]; // next[i] = biggest j < n with dist(i, j) < d
            next[n - 1] = n - 1;
            for i in (0..n - 1).rev() {
                let mut j = next[i + 1];
                while ps[i].dist(&ps[j]) >= d {
                    assert!(i < j);
                    j -= 1;
                }
                next[i] = j;
            }

            for i in 0..n {
                let mut j = i;
                for it in 0..k - 1 {
                    if j == n {
                        break;
                    }
                    j = next[j] + 1;
                }
                if j == n {
                    continue;
                }
                if ps[j].dist(&ps[i]) >= d {
                    return true;
                }
            }

            false
        };

        let mut L = 0;
        let mut R = side + 1;
        while L + 1 != R {
            let M = (L + R) / 2;
            if check(M) {
                L = M;
            } else {
                R = M;
            }
        }

        L
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![0,2],vec![2,0],vec![2,2],vec![0,0]], 4, 2)]
    #[case(2, vec![vec![0,0],vec![1,2],vec![2,0],vec![2,2],vec![2,1]], 4, 1)]
    #[case(2, vec![vec![0,0],vec![0,1],vec![0,2],vec![1,2],vec![2,0],vec![2,2],vec![2,1]], 5, 1)]
    #[case(6, vec![vec![2,0],vec![5,0],vec![0,0],vec![2,6]], 4, 2)]
    fn case(
        #[case] side: i32,
        #[case] points: Vec<Vec<i32>>,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_distance(side, points, k);
        assert_eq!(actual, expected);
    }
}
