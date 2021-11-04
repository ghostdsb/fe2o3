//Sieve of Eratosthenes
pub mod sieve {
    pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
        let mut sieve: Vec<bool> = vec![true].repeat((upper_bound + 1) as usize);

        for i in 2..=upper_bound {
            if sieve[i as usize] {
                let mut j = i * i;
                while j <= upper_bound {
                    sieve[j as usize] = false;
                    j += i;
                }
            }
        }

        sieve
            .iter()
            .enumerate()
            .filter(|(i, c)| **c && *i >= 2 as usize)
            .map(|(i, _c)| i as u64)
            .collect()
    }
}
