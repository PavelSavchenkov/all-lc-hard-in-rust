//! Solution for https://leetcode.com/problems/find-beautiful-indices-in-the-given-array-ii
//! 3008. Find Beautiful Indices in the Given Array II

fn calc_z(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut l = 0;
    let mut r = 0;
    let mut z = vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[i + z[i]] == s[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z[0] = n;
    z
}

fn get_occurences(s: &Vec<u8>, a: &Vec<u8>) -> Vec<usize> {
    let mut ss = a.clone();
    ss.extend(s.iter());
    let z = calc_z(&ss);
    let mut ans = Vec::new();
    for i in a.len()..a.len() + s.len() {
        if z[i] >= a.len() {
            ans.push(i - a.len());
        }
    }
    ans
}

impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let s = to_u8(&s);
        let a = to_u8(&a);
        let b = to_u8(&b);
        let k = k as usize;

        let a = get_occurences(&s, &a);
        let b = get_occurences(&s, &b);

        let mut ans = Vec::new();
        let mut ptr = 0;
        for &i in &a {
            while ptr < b.len() && b[ptr] <= i {
                ptr += 1;
            }
            let mut ok = false;
            if ptr < b.len() && b[ptr] - i <= k {
                ok = true;
            }
            if ptr > 0 && i - b[ptr - 1] <= k {
                ok = true;
            }
            if ok {
                ans.push(i as i32);
            }
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("isawsquirrelnearmysquirrelhouseohmy", "my", "squirrel", 15, vec![16,33])]
    #[case("abcd", "a", "a", 4, vec![0])]
    fn case(
        #[case] s: String,
        #[case] a: String,
        #[case] b: String,
        #[case] k: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::beautiful_indices(s, a, b, k);
        assert_eq!(actual, expected);
    }
}
