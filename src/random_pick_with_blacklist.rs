use rand::Rng;

struct Solution {
    len: u32,
    segs: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, mut blacklist: Vec<i32>) -> Self {
        blacklist.push(-1);
        blacklist.push(n);
        blacklist.sort();

        let mut segs = Vec::new();
        for i in 0..blacklist.len() - 1 {
            let start_real = blacklist[i] + 1;
            let cnt = blacklist[i + 1] - start_real;
            if cnt == 0 {
                continue;
            }
            let skipped = i as i32;
            let start_filtered = start_real - skipped;
            assert!(start_filtered >= 0);
            segs.push((start_filtered, start_real));
        }

        Self {
            len: n as u32 - (blacklist.len() as u32 - 2),
            segs,
        }
    }

    fn pick(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.len) as i32;
        let pos = self
            .segs
            .partition_point(|&(start_filtered, start_real)| start_filtered <= x)
            - 1;
        let (start_filtered, start_real) = self.segs[pos];
        start_real + (x - start_filtered)
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
