/***
 * Diffie-Hellman key exchange.
 * Alice and Bob use Diffie-Hellman key exchange to share secrets. They start with prime numbers, pick private keys, generate and share public keys, and then generate a shared secret key.
 *
 * Step 0
 * The test program supplies prime numbers p and g.
 *
 * Step 1
 * Alice picks a private key, a, greater than 1 and less than p. Bob does the same to pick a private key b.
 *
 * Step 2
 * Alice calculates a public key A.
 *
 *      A = g**a mod p
 *      Using the same p and g, Bob similarly calculates a public key B from his private key b.
 *
 * Step 3
 * Alice and Bob exchange public keys. Alice calculates secret key s.
 *      s = B**a mod p
 *
 * Bob calculates
 *      s = A**b mod p
 * The calculations produce the same result! Alice and Bob now share secret s.
*/
#[allow(unused)]
pub mod diffie_hellman {
    use rand::Rng;

    pub fn private_key(p: u64) -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(2..p)
    }

    pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
        power(g, a, p)
    }

    pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
        public_key(p, b_pub, a)
    }

    fn power(a: u64, b: u64, p: u64) -> u64 {
        let mut res = 1;
        let mut b = b;
        let mut a = a;
        a %= p;
        if a == 0 {
            return 0;
        }
        while b > 0 {
            if b % 2 == 1 {
                res = (res * a) % p;
            }
            b /= 2;
            a = (a * a) % p
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::diffie_hellman::*;

    #[test]
    fn test_private_key_in_range_key() {
        let primes: Vec<u64> = vec![
            5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
        ];
        let private_keys: Vec<u64> = primes.iter().map(|x| private_key(*x)).collect();

        for i in 0..primes.len() {
            assert!(1 < private_keys[i] && private_keys[i] < primes[i]);
        }
    }

    #[test]
    fn test_public_key_correct() {
        let p: u64 = 23;
        let g: u64 = 5;

        let private_key: u64 = 6;
        let expected: u64 = 8;

        assert_eq!(public_key(p, g, private_key), expected);
    }

    #[test]
    fn test_secret_key_correct() {
        let p: u64 = 11;

        let private_key_a = 7;
        let public_key_b = 8;
        let secret = secret(p, public_key_b, private_key_a);
        let expected = 2;

        assert_eq!(secret, expected);
    }

    #[test]
    fn test_public_key_correct_big_numbers() {
        let p: u64 = 4_294_967_299;

        let g: u64 = 8;

        let private_key: u64 = 4_294_967_296;

        let expected: u64 = 4096;

        assert_eq!(public_key(p, g, private_key), expected);
    }

    #[test]
    fn test_secret_key_correct_big_numbers() {
        let p: u64 = 4_294_967_927;

        let private_key_a = 4_294_967_300;

        let public_key_b = 843;

        let secret = secret(p, public_key_b, private_key_a);

        let expected = 1_389_354_282;

        assert_eq!(secret, expected);
    }

    // two biggest 64bit primes
    #[cfg(feature = "big-primes")]
    const PRIME_64BIT_1: u64 = 0xFFFF_FFFF_FFFF_FFC5;
    #[cfg(feature = "big-primes")]
    const PRIME_64BIT_2: u64 = 0xFFFF_FFFF_FFFF_FFAC;
    #[cfg(feature = "big-primes")]
    const PRIVATE_KEY_64BIT: u64 = 0xFFFF_FFFF_FFFF_FFC3;
    #[cfg(feature = "big-primes")]
    const PUBLIC_KEY_64BIT: u64 = 0xB851_EB85_1EB8_51C1;

    #[test]
    #[cfg(feature = "big-primes")]
    fn test_public_key_correct_biggest_numbers() {
        assert_eq!(
            public_key(PRIME_64BIT_1, PRIME_64BIT_2, PRIVATE_KEY_64BIT),
            PUBLIC_KEY_64BIT
        );
    }

    #[test]
    #[cfg(feature = "big-primes")]
    fn test_secret_key_correct_biggest_numbers() {
        let private_key_b = 0xEFFF_FFFF_FFFF_FFC0;
        let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);

        let expected_b = 4_340_425_873_327_658_043;
        assert_eq!(public_key_b, expected_b);

        let expected_key = 12_669_955_479_143_291_250;

        let secret_key = secret(PRIME_64BIT_1, public_key_b, PRIVATE_KEY_64BIT);

        assert_eq!(secret_key, expected_key);

        let secret_key = secret(PRIME_64BIT_1, PUBLIC_KEY_64BIT, private_key_b);

        assert_eq!(secret_key, expected_key);
    }

    #[test]
    #[cfg(feature = "big-primes")]
    fn test_changed_secret_key_biggest_numbers() {
        let private_key_a = private_key(PRIME_64BIT_1);
        let public_key_a = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_a);

        let private_key_b = private_key(PRIME_64BIT_1);
        let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);

        let secret_a = secret(PRIME_64BIT_1, public_key_b, private_key_a);
        let secret_b = secret(PRIME_64BIT_1, public_key_a, private_key_b);

        assert_eq!(secret_a, secret_b);
    }

    #[test]
    fn test_changed_secret_key() {
        let p: u64 = 13;
        let g: u64 = 11;

        let private_key_a = private_key(p);
        let private_key_b = private_key(p);

        let public_key_a = public_key(p, g, private_key_a);
        let public_key_b = public_key(p, g, private_key_b);

        // Key exchange
        let secret_a = secret(p, public_key_b, private_key_a);
        let secret_b = secret(p, public_key_a, private_key_b);

        assert_eq!(secret_a, secret_b);
    }
}
