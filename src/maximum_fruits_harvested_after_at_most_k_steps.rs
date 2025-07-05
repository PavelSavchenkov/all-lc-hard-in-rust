//! Solution for https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps
//! 2106. Maximum Fruits Harvested After at Most K Steps

#[derive(Debug)]
struct Fruit {
    pos: i32,
    cnt: i32,
}

fn solve_left(a: &Vec<Fruit>, start_pos: i32, max_steps: i32) -> i32 {
    let mut pref = vec![0; a.len() + 1];
    for i in 0..a.len() {
        pref[i + 1] = pref[i] + a[i].cnt;
    }

    let mut j_cut = 0;
    while j_cut < a.len() && a[j_cut].pos <= start_pos {
        j_cut += 1;
    }

    let mut ans = 0;
    // the first index we cannot take yet
    let mut j = 0;
    for i in 0..a.len() {
        if a[i].pos > start_pos {
            break;
        }
        let rem_right = max_steps - (start_pos - a[i].pos);
        if rem_right < 0 {
            continue;
        }
        j = j.max(i + 1);
        while j < a.len() && a[j].pos - a[i].pos <= rem_right {
            j += 1;
        }
        ans = ans.max(pref[j.max(j_cut)] - pref[i]);
    }
    ans
}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, mut start_pos: i32, k: i32) -> i32 {
        let mut fruits: Vec<_> = fruits
            .iter()
            .map(|v| Fruit {
                pos: v[0],
                cnt: v[1],
            })
            .collect();

        let mut ans = solve_left(&fruits, start_pos, k);
        fruits.reverse();
        for f in &mut fruits {
            f.pos = -f.pos;
        }
        start_pos = -start_pos;
        ans = ans.max(solve_left(&fruits, start_pos, k));

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
    #[case(vec![vec![2,8],vec![6,3],vec![8,6]], 5, 4, 9)]
    #[case(vec![vec![0,9],vec![4,1],vec![5,7],vec![6,2],vec![7,4],vec![10,9]], 5, 4, 14)]
    #[case(vec![vec![0,3],vec![6,4],vec![8,5]], 3, 2, 0)]
    fn case(
        #[case] fruits: Vec<Vec<i32>>,
        #[case] start_pos: i32,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_total_fruits(fruits, start_pos, k);
        assert_eq!(actual, expected);
    }
}
