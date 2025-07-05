//! Solution for https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits
//! 1505. Minimum Possible Integer After at Most K Adjacent Swaps On Digits

impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let a = to_u8(&num);
        let mut k = k as usize;
        let n = a.len();

        let mut poses = vec![Vec::new(); 10];
        for i in 0..n {
            let d = (a[i] - b'0') as usize;
            poses[d].push(i);
        }

        let calc_cost = |i: usize, j: usize, ptr: &mut Vec<usize>| -> usize {
            let mut cost = 0;
            for d in 0..10 {
                // [i; j)
                let k = poses[d][ptr[d]..].partition_point(|&pos| pos < j);
                cost += k;
            }
            cost
        };

        let mut ptr = vec![0; 10];
        let mut ans = Vec::new();
        for i in 0..n {
            for d in 0..10 {
                if ptr[d] < poses[d].len() {
                    let cost = calc_cost(i, poses[d][ptr[d]], &mut ptr);
                    if cost <= k {
                        ans.push(b'0' + d as u8);
                        k -= cost;
                        ptr[d] += 1;
                        break;
                    }
                }
            }
            assert!(ans.len() == i + 1);
        }

        from_u8(&ans)
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
    #[case("4321", 4, "1342")]
    #[case("100", 1, "010")]
    #[case("36789", 1000, "36789")]
    fn case(#[case] num: String, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::min_integer(num, k);
        assert_eq!(actual, expected);
    }
}
