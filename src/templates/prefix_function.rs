fn calc_prefix_function(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            p[i] = j + 1;
        }
    }
    p
}