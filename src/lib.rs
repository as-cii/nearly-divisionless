use rand::Rng;

// Fast Random Integer Generation in an Interval by Daniel Lemire
// (https://arxiv.org/abs/1805.10941)
pub fn gen_range<R: Rng>(rng: &mut R, lo: u64, hi: u64) -> u64 {
    let s = hi - lo + 1;
    let mut x: u64 = rng.gen();
    let mut m = s as u128 * x as u128;
    let mut l = m as u64;
    if l < s {
        let t = (-(s as i64) as u64) % s;
        while l < t {
            x = rng.gen();
            m = s as u128 * x as u128;
            l = m as u64;
        }
    }
    lo + (m >> 64) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_and_variance() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let lo = rng.gen::<u16>() as u64;
            let hi = lo + rng.gen::<u16>() as u64;

            const BUCKETS_LEN: usize = 5000000;
            let mut buckets = Vec::new();
            buckets.resize((hi - lo + 1) as usize, 0);
            for _ in 0..BUCKETS_LEN {
                let n = gen_range(&mut rng, lo, hi) - lo;
                buckets[n as usize] += 1;
            }

            let sum: u64 = buckets
                .iter()
                .enumerate()
                .map(|(i, k)| (k * (i as u64 + lo)))
                .sum();

            let expected_mean = (lo + hi) as f64 / 2_f64;
            let actual_mean = sum as f64 / BUCKETS_LEN as f64;
            let mean_error =
                1_f64 - (actual_mean.min(expected_mean) / actual_mean.max(expected_mean));
            assert!(mean_error < 0.001);

            let expected_var = ((hi - lo + 1).pow(2) - 1) as f64 / 12_f64;
            let mut actual_var = 0_f64;
            for (i, k) in buckets.iter().enumerate() {
                let n = (i as u64 + lo) as f64;
                actual_var += *k as f64 * (n.max(actual_mean) - n.min(actual_mean)).powf(2_f64);
            }
            actual_var /= BUCKETS_LEN as f64;
            let var_error = 1_f64 - (actual_var.min(expected_var) / actual_var.max(expected_var));
            assert!(var_error < 0.002);
        }
    }
}
