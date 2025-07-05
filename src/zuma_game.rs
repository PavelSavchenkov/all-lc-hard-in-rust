//! Solution for https://leetcode.com/problems/zuma-game
//! 488. Zuma Game

const B: usize = 3;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct BitVec {
    a: u64,
    len: usize,
}

impl BitVec {
    fn new() -> Self {
        Self { a: 0, len: 0 }
    }

    fn new_from_vec(a: &Vec<u64>) -> Self {
        let mut this = Self::new();
        for i in 0..a.len() {
            this = this.insert(i, a[i]);
        }
        this
    }

    fn insert(&self, pos: usize, val: u64) -> Self {
        assert!(val < (1 << B));
        assert!(pos <= self.len);
        let a;
        if pos == self.len {
            a = self.a ^ (val << (pos * B));
        } else {
            let up = self.a << B;
            let len_down = pos * B;
            let mask_down = (1 << len_down) - 1;
            let mask_up = !((1 << (len_down + B)) - 1);
            a = (up & mask_up) ^ (val << len_down) ^ (self.a & mask_down);
        }
        Self {
            a,
            len: self.len + 1,
        }
    }

    fn get(&self, pos: usize) -> u64 {
        (self.a >> (pos * B)) & ((1 << B) - 1)
    }

    // [l; r)
    fn remove(&self, mut l: usize, mut r: usize) -> Self {
        let len = self.len - (r - l);
        l *= B;
        r *= B;
        let mask_down = (1 << l) - 1;
        let a = ((self.a >> (r - l)) & !mask_down) ^ (self.a & mask_down);
        Self { a, len }
    }

    fn try_reduce(&mut self, mut pos: usize) {
        while pos < self.len {
            let val = self.get(pos);
            let mut l = pos;
            while l > 0 && self.get(l - 1) == val {
                l -= 1;
            }
            let mut r = pos;
            while r + 1 < self.len && self.get(r + 1) == val {
                r += 1;
            }
            let len = r - l + 1;
            if len < 3 {
                return;
            }
            assert!(len <= self.len);
            *self = self.remove(l, r + 1);
            pos = l;
        }
    }
}

fn encode(ch: u8) -> u64 {
    "RYBGW".as_bytes().iter().position(|&c| c == ch).unwrap() as u64
}

use std::collections::HashMap;

fn go(board: BitVec, hand: BitVec, map: &mut HashMap<(BitVec, BitVec), usize>) -> usize {
    let key = (board, hand);
    if let Some(&ans) = map.get(&key) {
        return ans;
    }
    if board.len == 0 {
        return 0;
    }
    if hand.len == 0 {
        return usize::MAX;
    }

    let mut ans = usize::MAX;
    let mut prev = u64::MAX;
    for i in 0..hand.len {
        let cur = hand.get(i);
        if cur == prev {
            continue;
        }
        for j in 0..=board.len {
            let mut new_board = board.insert(j, cur);
            new_board.try_reduce(j);
            let new_hand = hand.remove(i, i + 1);
            let cur_ans = go(new_board, new_hand, map);
            if cur_ans < usize::MAX {
                ans = ans.min(cur_ans + 1);
            }
        }
        prev = cur;
    }

    map.insert((board, hand), ans);
    ans
}

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let board = to_u8(&board);
        let hand = to_u8(&hand);

        let board: Vec<_> = board.iter().map(|&c| encode(c)).collect();
        let mut hand: Vec<_> = hand.iter().map(|&c| encode(c)).collect();
        hand.sort();

        let board = BitVec::new_from_vec(&board);
        let hand = BitVec::new_from_vec(&hand);

        let mut map = HashMap::new();
        let ans = go(board, hand, &mut map);
        if ans == usize::MAX {
            return -1;
        }
        ans as i32
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
    #[case("WRRBBW", "RB", -1)]
    #[case("WWRRBBWW", "WRBRW", 2)]
    #[case("G", "GGGGG", 2)]
    #[case("BGBBYRYYRBRWYBRR", "YWYRB", -1)]
    fn case(#[case] board: String, #[case] hand: String, #[case] expected: i32) {
        let actual = Solution::find_min_step(board, hand);
        assert_eq!(actual, expected);
    }
}
