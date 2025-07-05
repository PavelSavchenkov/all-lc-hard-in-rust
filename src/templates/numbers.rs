
fn get_prime_sieve(n: usize) -> Vec<bool> {
    let mut is_p = vec![true; n + 1];
    is_p[0] = false;
    is_p[1] = false;
    for p in 2..=n {
        if !is_p[p] {
            continue;
        }
        for i in (p + p..=n).step_by(p) {
            is_p[i] = false;
        }
    }
    is_p
}

fn factorize(mut n: u32) -> Vec<(u32, u32)> {
    let mut factors = Vec::new();
    let mut i = 2;
    while i * i <= n {
        let p = i;
        let mut k = 0;
        while n % p == 0 {
            n /= p;
            k += 1;
        }
        if k > 0 {
            factors.push((p, k));
        }
        i += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}