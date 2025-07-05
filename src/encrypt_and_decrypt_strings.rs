//! Solution for https://leetcode.com/problems/encrypt-and-decrypt-strings
//! 2227. Encrypt and Decrypt Strings

const A: usize = 26;

fn encode_char(ch: u8) -> usize {
    (ch - b'a') as usize
}

struct Encrypter {
    keys: Vec<u8>,
    values: Vec<Vec<u8>>,
    dict: Vec<Vec<u8>>,
    allowed: Vec<Vec<Vec<bool>>>, // key, value0, value1
    pos_key: Vec<Option<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let keys: Vec<_> = keys.iter().map(|&c| c as u8).collect();
        let values: Vec<_> = to_u8_vec(&values);
        let dict: Vec<_> = to_u8_vec(&dictionary);

        let mut pos_key = vec![None; A];
        let mut allowed = vec![vec![vec![false; A]; A]; A];
        for i in 0..keys.len() {
            let key = encode_char(keys[i]);
            allowed[key][encode_char(values[i][0])][encode_char(values[i][1])] = true;
            pos_key[key] = Some(i);
        }
        Self {
            keys,
            values,
            dict,
            allowed,
            pos_key,
        }
    }

    fn encrypt(&self, s: String) -> String {
        let s = to_u8(&s);
        let mut res = Vec::with_capacity(s.len() * 2);
        for &ch in &s {
            let c = encode_char(ch);
            if let Some(i) = self.pos_key[c] {
                res.extend(self.values[i].iter());
            } else {
                return String::new();
            }
        }
        from_u8(&res)
    }

    fn decrypt(&self, s: String) -> i32 {
        let s = to_u8(&s);

        let mut ans = 0;
        for i in 0..self.dict.len() {
            if self.dict[i].len() * 2 != s.len() {
                continue;
            }
            let mut fail = false;
            for j in (0..s.len()).step_by(2) {
                let key = encode_char(self.dict[i][j / 2]);
                let value0 = encode_char(s[j]);
                let value1 = encode_char(s[j + 1]);
                if !self.allowed[key][value0][value1] {
                    fail = true;
                    break;
                }
            }
            if !fail {
                ans += 1;
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

/**
 * Your Encrypter object will be instantiated and called as such:
 * let obj = Encrypter::new(keys, values, dictionary);
 * let ret_1: String = obj.encrypt(word1);
 * let ret_2: i32 = obj.decrypt(word2);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
