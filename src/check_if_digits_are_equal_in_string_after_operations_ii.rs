//! Solution for https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-ii
//! 3463. Check If Digits Are Equal in String After Operations II

struct Binom {
    b: Vec<Vec<usize>>,
    p: usize, // p must be prime
}

impl Binom {
    fn new(p: usize) -> Self {
        assert!(p >= 2);
        let mut b = vec![vec![0; p]; p];
        b[0][0] = 1;
        for n in 1..p {
            b[n][0] = 1;
            for k in 1..=n {
                b[n][k] = (b[n - 1][k] + b[n - 1][k - 1]) % p;
            }
        }
        Self { b, p }
    }

    fn get(&self, mut n: usize, mut k: usize) -> usize {
        if n < k {
            return 0;
        }
        let mut ans = 1;
        while n > 0 {
            let dig_n = n % self.p;
            n /= self.p;
            let dig_k = k % self.p;
            k /= self.p;
            ans = (ans * self.b[dig_n][dig_k]) % self.p;
        }
        ans
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

fn crt(r2: usize, r5: usize) -> usize {
    for r in 0..10 {
        if r % 2 == r2 && r % 5 == r5 {
            return r;
        }
    }
    unreachable!()
}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let s = to_u8(&s);
        let n = s.len();

        let binom2 = Binom::new(2);
        let binom5 = Binom::new(5);
        let mut c0 = 0;
        for i in 0..n - 1 {
            let coef = (s[i] - b'0') as usize;
            let b2 = binom2.get(n - 2, i);
            let b5 = binom5.get(n - 2, i);
            let b10 = crt(b2, b5);
            c0 = (c0 + b10 * coef) % 10;
        }
        let mut c1 = 0;
        for i in 1..n {
            let coef = (s[i] - b'0') as usize;
            let b2 = binom2.get(n - 2, i - 1);
            let b5 = binom5.get(n - 2, i - 1);
            let b10 = crt(b2, b5);
            c1 = (c1 + b10 * coef) % 10;
        }
        c0 == c1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("3902", true)]
    #[case("34789", false)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::has_same_digits(s);
        assert_eq!(actual, expected);
    }
}
