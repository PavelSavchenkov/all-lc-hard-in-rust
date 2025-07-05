//! Solution for https://leetcode.com/problems/replace-non-coprime-numbers-in-array
//! 2197. Replace Non-Coprime Numbers in Array

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut st = Vec::new();
        for &x in &nums {
            st.push(x);
            loop {
                if st.len() < 2 {
                    break;
                }
                let a = st[st.len() - 1];
                let b = st[st.len() - 2];
                let g = gcd(a, b);
                if g == 1 {
                    break;
                }
                let ab = lcm(a, b);
                st.pop();
                st.pop();
                st.push(ab);
            }
        }

        st
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![6,4,3,2,7,6,2], vec![12,7,6])]
    #[case(vec![2,2,1,1,3,3,3], vec![2,1,1,3])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::replace_non_coprimes(nums);
        assert_eq!(actual, expected);
    }
}
