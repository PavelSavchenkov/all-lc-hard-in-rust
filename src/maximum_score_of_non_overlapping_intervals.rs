//! Solution for https://leetcode.com/problems/maximum-score-of-non-overlapping-intervals
//! 3414. Maximum Score of Non-overlapping Intervals

#[derive(Clone, Copy)]
struct Seg {
    l: i32,
    r: i32,
    w: i64,
    id: usize,
}

impl Seg {
    fn new(v: &Vec<i32>, id: usize) -> Self {
        Self {
            l: v[0],
            r: v[1],
            w: v[2] as i64,
            id,
        }
    }
}

fn get_dp(mut segs: Vec<Seg>, max_cnt: usize) -> Vec<Vec<i64>> {
    let mut xs = Vec::new();
    for s in &segs {
        assert!(s.l <= s.r);
        xs.push(s.l);
        xs.push(s.r);
    }
    xs.sort();
    xs.dedup();

    segs.sort_by_key(|s| s.r);

    let mut dp = vec![vec![0; xs.len() + 1]; max_cnt + 1];
    for cnt in 1..=max_cnt {
        for id_x in 0..xs.len() {
            let i = segs.partition_point(|s| s.r < xs[id_x]);
            let j = segs.partition_point(|s| s.r <= xs[id_x]);
            let mut best = i64::MIN;
            for k in i..j {
                let l = segs[k].l;
                let id_l = xs.binary_search(&l).unwrap();
                best = best.max(dp[cnt - 1][id_l] + segs[k].w);
            }
            let prev = dp[cnt][id_x];
            let prev2 = dp[cnt - 1][id_x + 1];
            dp[cnt][id_x + 1] = best.max(prev).max(prev2);
        }
    }
    dp
}

fn solve(mut segs: Vec<Seg>, cnt: usize) -> Vec<usize> {
    if segs.is_empty() || cnt == 0 {
        return Vec::new();
    }
    let dp_pref = get_dp(segs.clone(), cnt - 1);
    for s in &mut segs {
        let l = s.l;
        let r = s.r;
        s.l = -r;
        s.r = -l;
    }
    let mut dp_suff = get_dp(segs.clone(), cnt - 1);
    for j in 0..dp_suff.len() {
        dp_suff[j].reverse();
    }

    let mut xs = Vec::new();
    for s in &mut segs {
        let l = s.l;
        let r = s.r;
        s.l = -r;
        s.r = -l;
        xs.push(s.l);
        xs.push(s.r);
    }

    let mut max_w = i64::MIN;
    let mut min_id = 0;
    let mut L = 0;
    let mut R = 0;
    xs.sort();
    xs.dedup();
    for s in &segs {
        let l = xs.binary_search(&s.l).unwrap();
        let r = xs.binary_search(&s.r).unwrap();
        for left in 0..cnt {
            let right = cnt - 1 - left;
            let cur = dp_pref[left][l] + dp_suff[right][r + 1] + s.w;
            if cur > max_w || (cur == max_w && s.id < min_id) {
                max_w = cur;
                min_id = s.id;
                L = s.l;
                R = s.r;
            }
        }
    }
    assert!(max_w > i64::MIN);

    let mut new_segs = Vec::new();
    for &s in &segs {
        if s.r < L || R < s.l {
            new_segs.push(s);
        }
    }

    let mut rec = solve(new_segs, cnt - 1);
    rec.insert(0, min_id);
    rec
}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let segs: Vec<_> = (0..intervals.len())
            .map(|i| Seg::new(&intervals[i], i))
            .collect();

        let ids = solve(segs, 4);
        ids.iter().map(|&id| id as i32).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3,2],vec![4,5,2],vec![1,5,5],vec![6,9,3],vec![6,7,1],vec![8,9,1]], vec![2,3])]
    #[case(vec![vec![5,8,1],vec![6,7,7],vec![4,7,3],vec![9,10,6],vec![7,8,2],vec![11,14,3],vec![3,5,5]], vec![1,3,5,6])]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::maximum_weight(intervals);
        assert_eq!(actual, expected);
    }
}
