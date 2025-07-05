//! Solution for https://leetcode.com/problems/smallest-palindromic-rearrangement-ii
//! 3518. Smallest Palindromic Rearrangement II

fn get_primes(n: usize) -> Vec<i64> {
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();
    for p in 2..=n {
        if is_prime[p] {
            primes.push(p as i64);
            for x in (p + p..=n).step_by(p) {
                is_prime[x] = false;
            }
        }
    }
    primes
}

fn get_deg(mut num: i64, p: i64) -> i64 {
    let mut ans = 0;
    while num > 0 {
        num /= p;
        ans += num;
    }
    ans
}

const A: usize = 26;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let s = to_u8(&s);
        let n = s.len();
        let mut k = k as i64;

        let primes = get_primes(n);

        let cnt_ways = |cnt: &Vec<usize>, k: i64| -> i64 {
            let len = cnt.iter().fold(0, |acc, e| acc + *e) as i64;
            assert!(len <= n as i64);
            let mut ways = 1;
            for &p in &primes {
                let mut pw = get_deg(len, p);
                for c in 0..A {
                    pw -= get_deg(cnt[c] as i64, p);
                }
                assert!(pw >= 0);
                for it in 0..pw {
                    ways = i64::saturating_mul(ways, p);
                    if ways >= k {
                        return k;
                    }
                }
            }
            ways
        };

        let mut cnt = vec![0; A];
        for i in 0..n / 2 {
            let c = (s[i] - b'a') as usize;
            cnt[c] += 1;
        }
        let mut ans = vec![0; n];
        for i in 0..n / 2 {
            for c in 0..A {
                if cnt[c] > 0 {
                    cnt[c] -= 1;
                    let ways = cnt_ways(&cnt, k);
                    if ways < k {
                        k -= ways;
                    } else {
                        ans[i] = b'a' + c as u8;
                        break;
                    }
                    cnt[c] += 1;
                }
            }
            if ans[i] == 0 {
                return "".to_string();
            }
            ans[n - 1 - i] = ans[i];
        }
        if n % 2 == 1 {
            ans[n / 2] = s[n / 2];
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
    #[case("abba", 2, "baab")]
    #[case("aa", 2, "")]
    #[case("bacab", 1, "abcba")]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::smallest_palindrome(s, k);
        assert_eq!(actual, expected);
    }
}
