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
