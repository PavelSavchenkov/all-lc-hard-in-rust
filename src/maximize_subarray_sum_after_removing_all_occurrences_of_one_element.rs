//! Solution for https://leetcode.com/problems/maximize-subarray-sum-after-removing-all-occurrences-of-one-element
//! 3410. Maximize Subarray Sum After Removing All Occurrences of One Element

struct SegmTreeMin {
    t: Vec<i64>,
    sz: usize,
}

impl SegmTreeMin {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        Self {
            t: vec![i64::MAX; sz * 2],
            sz,
        }
    }

    fn get_min(&self, mut l: usize, mut r: usize) -> i64 {
        l += self.sz;
        r += self.sz;
        let mut ans = i64::MAX;
        while l <= r {
            ans = ans.min(self.t[l]);
            ans = ans.min(self.t[r]);
            l = (l + 1) / 2;
            r = (r - 1) / 2;
        }
        ans
    }

    fn update(&mut self, pos: usize, val: i64) {
        let mut v = self.sz + pos;
        self.t[v] = val;
        v /= 2;
        while v > 0 {
            self.t[v] = self.t[v * 2].min(self.t[v * 2 + 1]);
            v /= 2;
        }
    }
}

impl Solution {
    pub fn max_subarray_sum(a: Vec<i32>) -> i64 {
        let max = *a.iter().max().unwrap();
        if max <= 0 {
            return max as i64;
        }

        let n = a.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + a[i] as i64;
        }
        let mut pref_tree = SegmTreeMin::new(pref.len());
        for i in 0..pref.len() {
            pref_tree.update(i, pref[i]);
        }

        let mut vals = a.clone();
        vals.sort();
        vals.dedup();

        let mut last_pos = vec![n; vals.len()];
        let mut cnt = vec![0; vals.len()];
        let mut min_pref_without = vec![i64::MAX; vals.len()];
        let mut ans = 0;
        let mut suff = 0;
        for i in 0..n {
            let v = vals.binary_search(&a[i]).unwrap();
            suff = suff.max(0) + a[i] as i64;

            // now update suff for cases when we remove a[i]
            let start = if cnt[v] == 0 { 0 } else { last_pos[v] };
            min_pref_without[v] =
                min_pref_without[v].min(pref_tree.get_min(start, i) - a[i] as i64 * cnt[v]);
            cnt[v] += 1;
            last_pos[v] = i;
            suff = suff.max(pref[i + 1] - a[i] as i64 * cnt[v] - min_pref_without[v]);

            ans = ans.max(suff);
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
    #[case(vec![-3,2,-2,-1,3,-2,3], 7)]
    #[case(vec![1,2,3,4], 10)]
    #[case(vec![1,2,3,-1,10,-1,20], 36)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::max_subarray_sum(nums);
        assert_eq!(actual, expected);
    }
}
