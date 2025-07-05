//! Solution for https://leetcode.com/problems/maximum-number-of-groups-getting-fresh-donuts
//! 1815. Maximum Number of Groups Getting Fresh Donuts

use std::collections::HashMap;

fn rec(
    cnt: &mut Vec<usize>,
    pref_sum: usize,
    mut start_rem: usize,
    mem: &mut HashMap<u64, usize>,
) -> usize {
    // let H = get_hash(&vec![cnt.clone(), vec![pref_sum, start_rem]]);
    // if let Some(res) = mem.get(&H) {
    //     return *res;
    // }

    let mut ans = 0;
    if pref_sum == 0 {
        ans += 1;
        start_rem = 0;
    }

    let mut best_rec = 0;
    for rem in start_rem..cnt.len() {
        if cnt[rem] > 0 {
            cnt[rem] -= 1;
            let cur = rec(cnt, (pref_sum + rem) % cnt.len(), rem, mem);
            best_rec = best_rec.max(cur);
            cnt[rem] += 1;
        }
    }
    ans += best_rec;
    // mem.insert(H, ans);
    ans
}

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let batch_size = batch_size as usize;
        let groups: Vec<_> = groups.iter().map(|&x| x as usize).collect();

        let mut cnt = vec![0; batch_size];
        for &g in &groups {
            cnt[g % batch_size] += 1;
        }

        let mut ans = 0;
        ans += cnt[0];
        cnt[0] = 0;

        loop {
            let mut found = false;
            for r0 in 0..batch_size {
                let r1 = (batch_size - r0) % batch_size;
                if (r0 == r1 && cnt[r0] >= 2) || (r0 != r1 && cnt[r0] >= 1 && cnt[r1] >= 1) {
                    cnt[r0] -= 1;
                    cnt[r1] -= 1;
                    found = true;
                    ans += 1;
                }
            }
            if !found {
                break;
            }
        }

        let mut mem = HashMap::new();
        ans += rec(&mut cnt, 0, 0, &mut mem);
        if groups.iter().sum::<usize>() % batch_size == 0 {
            ans -= 1;
        }
        ans as i32
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![1,2,3,4,5,6], 4)]
    #[case(4, vec![1,3,2,5,2,2,1,6], 4)]
    fn case(#[case] batch_size: i32, #[case] groups: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_happy_groups(batch_size, groups);
        assert_eq!(actual, expected);
    }
}
