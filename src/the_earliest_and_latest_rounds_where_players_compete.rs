//! Solution for https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete
//! 1900. The Earliest and Latest Rounds Where Players Compete

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let mut n = n as usize;
        let first = (first_player - 1) as usize;
        let second = (second_player - 1) as usize;

        let mut min_round = i32::MAX;
        let mut max_round = i32::MIN;
        let mut pairs = vec![(first, second)];
        let mut round = 1;
        while !pairs.is_empty() {
            let mut new_pairs = Vec::new();
            for &(f, s) in &pairs {
                assert!(f < n);
                assert!(s < n);
                assert!(f < s);
                if f == n - s - 1 {
                    min_round = min_round.min(round);
                    max_round = max_round.max(round);
                    continue;
                }
                let half = n / 2;
                for mask in 0..((1 << half) as usize) {
                    let mut new_f = 0;
                    let mut new_s = 0;
                    let mut bad_mask = false;
                    for i in 0..half {
                        if ((mask >> i) & 1) == 1 {
                            if n - i - 1 == f || n - i - 1 == s {
                                bad_mask = true;
                                break;
                            }
                            if i < f {
                                new_f += 1;
                            }
                            if i < s {
                                new_s += 1;
                            }
                        } else {
                            if i == s || i == f {
                                bad_mask = true;
                                break;
                            }
                            if n - i - 1 < f {
                                new_f += 1;
                            }
                            if n - i - 1 < s {
                                new_s += 1;
                            }
                        }
                    }
                    if bad_mask {
                        continue;
                    }
                    if n % 2 == 1 {
                        let mid = n / 2;
                        if mid < f {
                            new_f += 1;
                        }
                        if mid < s {
                            new_s += 1;
                        }
                    }
                    new_pairs.push((new_f, new_s));
                }
            }
            new_pairs.sort();
            new_pairs.dedup();
            pairs = new_pairs;
            round += 1;
            n = (n + 1) / 2;
        }

        vec![min_round, max_round]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(11, 2, 4, vec![3,4])]
    #[case(5, 1, 5, vec![1,1])]
    fn case(
        #[case] n: i32,
        #[case] first_player: i32,
        #[case] second_player: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::earliest_and_latest(n, first_player, second_player);
        assert_eq!(actual, expected);
    }
}
