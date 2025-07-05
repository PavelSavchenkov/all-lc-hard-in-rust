//! Solution for https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii
//! 3197. Find the Minimum Area to Cover All Ones II

fn bit(mask: usize, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

// set bits [l, r]
fn mask_seg(l: usize, r: usize) -> usize {
    (1 << (r + 1)) - 1 - ((1 << l) - 1)
}

impl Solution {
    pub fn minimum_sum(g: Vec<Vec<i32>>) -> i32 {
        let n = g.len();
        let m = g[0].len();

        // --> (cnt ones, area)
        let get_bbox = |r0: usize, r1: usize, c0: usize, c1: usize| -> (usize, usize) {
            // [r0, r1), [c0, c1)
            let mut r_min = n;
            let mut r_max = 0;
            let mut c_min = m;
            let mut c_max = 0;
            let mut ones = 0;
            for r in r0..r1 {
                for c in c0..c1 {
                    if g[r][c] == 1 {
                        ones += 1;
                        r_min = r_min.min(r);
                        r_max = r_max.max(r);
                        c_min = c_min.min(c);
                        c_max = c_max.max(c);
                    }
                }
            }
            if r_min > r_max {
                return (0, 0);
            }
            (ones, (r_max - r_min + 1) * (c_max - c_min + 1))
        };

        let mut mask_col = vec![0; m];
        let mut cnt_ones = 0;
        let mut r_min = n;
        let mut r_max = 0;
        for r in 0..n {
            for c in 0..m {
                if g[r][c] == 1 {
                    r_min = r_min.min(r);
                    r_max = r_max.max(r);
                    cnt_ones += 1;
                    mask_col[c] |= 1 << r;
                }
            }
        }

        assert!(r_min <= r_max);

        let mut ans = usize::MAX;
        for h1 in 1..=r_max - r_min + 1 {
            let mut L1 = m;
            let mut R1 = 0;
            for r in r_min..r_min + h1 {
                for c in 0..m {
                    if g[r][c] == 1 {
                        L1 = L1.min(c);
                        R1 = R1.max(c);
                    }
                }
            }
            assert!(L1 <= R1);

            for l1 in L1..=R1 {
                let mut inside_cnt = 0;
                for r1 in l1..=R1 {
                    for r in r_min..r_min + h1 {
                        if g[r][r1] == 1 {
                            inside_cnt += 1;
                        }
                    }
                    if L1 < l1 && r1 < R1 {
                        let (left_cnt, left_area) = get_bbox(r_min, r_min + h1, 0, l1);
                        let (right_cnt, right_area) = get_bbox(r_min, r_min + h1, r1 + 1, R1 + 1);
                        if left_cnt + right_cnt + inside_cnt != cnt_ones {
                            continue;
                        }
                        let cur = h1 * (r1 - l1 + 1) + left_area + right_area;
                        ans = ans.min(cur);
                    } else {
                        // n**2
                        for h2 in 1..=r_max - r_min + 1 {
                            // n**3
                            let mut L2 = m;
                            let mut R2 = 0;
                            for r in r_max + 1 - h2..=r_max {
                                for c in 0..m {
                                    if g[r][c] == 1 {
                                        if r >= r_min + h1 || c < l1 || r1 < c {
                                            L2 = L2.min(c);
                                            R2 = R2.max(c);
                                        }
                                    }
                                }
                            }
                            if L2 > R2 {
                                continue;
                            }
                            for l2 in L2..=R2 {
                                for r2 in l2..=R2 {
                                    if l2 != L2 && r2 != R2 {
                                        continue;
                                    }
                                    // n**4
                                    if h1 + h2 > r_max - r_min + 1 {
                                        if l1.max(l2) <= r1.min(r2) {
                                            continue;
                                        }
                                    }
                                    let mut mask_cover = [0; 32];
                                    let mask1 = mask_seg(r_min, r_min + h1 - 1);
                                    for c in l1..=r1 {
                                        mask_cover[c] |= mask1;
                                    }
                                    let mask2 = mask_seg(r_max + 1 - h2, r_max);
                                    for c in l2..=r2 {
                                        mask_cover[c] |= mask2;
                                    }
                                    let mut l3 = m;
                                    let mut r3 = 0;
                                    let mut l4 = n;
                                    let mut r4 = 0;
                                    for c in 0..m {
                                        let mask_remain = (mask_col[c] & !mask_cover[c]) as i32;
                                        if mask_remain != 0 {
                                            l3 = l3.min(c);
                                            r3 = r3.max(c);
                                            let hb = 31 - mask_remain.leading_zeros() as usize;
                                            let lb = mask_remain.trailing_zeros() as usize;
                                            assert!(lb <= hb);
                                            l4 = l4.min(lb);
                                            r4 = r4.max(hb);
                                        }
                                    }
                                    if l3 > r3 {
                                        break;
                                    }
                                    let cur = h1 * (r1 - l1 + 1)
                                        + h2 * (r2 - l2 + 1)
                                        + (r4 - l4 + 1) * (r3 - l3 + 1);
                                    let mut not_inter = true;
                                    for c in l3..=r3 {
                                        let mask = mask_seg(l4, r4);
                                        if (mask_cover[c] & mask) != 0 {
                                            not_inter = false;
                                            break;
                                        }
                                    }
                                    if not_inter {
                                        ans = ans.min(cur);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,0,1],vec![1,1,1]], 5)]
    #[case(vec![vec![1,0,1,0],vec![0,1,0,1]], 5)]
    #[case(vec![vec![0,1,1],vec![0,1,0]], 3)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_sum(grid);
        assert_eq!(actual, expected);
    }
}
