#[derive(Clone)]
struct FenwTreeSum {
    t: Vec<i64>,
}

impl FenwTreeSum {
    fn new(n: usize) -> Self {
        Self { t: vec![0; n] }
    }

    fn add_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] += val;
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_sum_pref(&self, r: usize) -> i64 {
        if r == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut i = r - 1;
        loop {
            sum += self.t[i];
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        sum
    }

    fn get_sum(&self, l: usize, r: usize) -> i64 {
        if l >= r {
            return 0;
        }
        self.get_sum_pref(r) - self.get_sum_pref(l)
    }
}

struct FenwTreeMax {
    t: Vec<i64>,
}

impl FenwTreeMax {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        let mut i = pos;
        while i < self.t.len() {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
        }
    }

    // [0; r)
    fn get_max(&mut self, r: usize) -> i64 {
        let mut mx = i64::MIN;
        if r == 0 {
            return mx;
        }
        let mut i = r - 1;
        loop {
            mx = mx.max(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        mx
    }
}

#[derive(Clone)]
struct FenwTreeMinSuff {
    t: Vec<i64>,
}

impl FenwTreeMinSuff {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        assert!(pos < self.t.len());
        let mut i = self.t.len() - pos - 1;
        while i < self.t.len() {
            self.t[i] = self.t[i].min(val);
            i |= i + 1;
        }
    }

    // [r, t.len())
    fn get_min(&mut self, r: usize) -> i64 {
        let mut ans = i64::MAX;
        if r == self.t.len() {
            return ans;
        }
        let mut i = self.t.len() - r - 1;
        loop {
            ans = ans.min(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        ans
    }
}

#[derive(Clone)]
struct FenwTreeMaxSuff {
    t: Vec<i64>,
}

impl FenwTreeMaxSuff {
    fn new(n: usize, val: i64) -> Self {
        Self { t: vec![val; n] }
    }

    fn relax_point(&mut self, pos: usize, val: i64) {
        assert!(pos < self.t.len());
        let mut i = self.t.len() - pos - 1;
        while i < self.t.len() {
            self.t[i] = self.t[i].max(val);
            i |= i + 1;
        }
    }

    // [r, t.len())
    fn get_max(&mut self, r: usize) -> i64 {
        let mut ans = i64::MIN;
        if r == self.t.len() {
            return ans;
        }
        let mut i = self.t.len() - r - 1;
        loop {
            ans = ans.max(self.t[i]);
            i &= i + 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        ans
    }
}
