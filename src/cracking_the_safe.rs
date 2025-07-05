//! Solution for https://leetcode.com/problems/cracking-the-safe
//! 753. Cracking the Safe

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;

        if n == 1 {
            let mut str = String::new();
            for c in 0..k {
                str.push((b'0' + c as u8) as char);
            }
            return str;
        }

        let N = usize::pow(k, (n - 1) as u32);
        let mut g = vec![Vec::new(); N];
        for v in 0..N {
            for c in 0..k {
                let u = (v * k + c) % N;
                g[v].push(u);
            }
        }

        let mut st = Vec::new();
        let mut ans = Vec::new();
        st.push(0);
        while !st.is_empty() {
            let v = *st.last().unwrap();
            if g[v].is_empty() {
                ans.push(v);
                st.pop();
            } else {
                let u = g[v].pop().unwrap();
                st.push(u);
            }
        }
        ans.reverse();
        ans.pop();

        let mut str = String::new();
        for v in &ans {
            if str.is_empty() {
                str.push_str(&format!("{:0>width$}", v, width = n));
            } else {
                str.push((b'0' + (v % k) as u8) as char);
            }
        }

        str
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 2, "10")]
    #[case(2, 2, "01100")]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::crack_safe(n, k);
        assert_eq!(actual, expected);
    }
}
