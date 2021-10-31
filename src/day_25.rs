/**
 * The Collatz Conjecture or 3x+1 problem can be summarized as follows:
 * Take any positive integer n. If n is even, divide n by 2 to get n / 2.
 * If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely.
 *
 * The conjecture states that no matter which number you start with, you will always reach 1 eventually.
 * But sometimes the number grow significantly before it reaches 1.
 * This can lead to an integer overflow and makes the algorithm unsolvable within the range of a number in u64.
 * Given a number n, return the number of steps required to reach 1.
*/

#[allow(unused)]

pub mod collatz_conjecture {
    pub fn collatz(n: u64) -> Option<u64> {
        let mut x = n;
        let mut steps = 0;
        while x != 1 {
            if x >= u64::MAX / 3 && x % 2 == 1 || x == 0 {
                return None;
            }
            x = match x % 2 == 0 {
                true => x / 2,
                false => 3 * x + 1,
            };
            steps += 1;
        }
        Some(steps)
    }
}

#[cfg(test)]
mod test {
    use super::collatz_conjecture::*;
    #[test]
    fn test_1() {
        assert_eq!(Some(0), collatz(1));
    }
    #[test]
    fn test_16() {
        assert_eq!(Some(4), collatz(16));
    }
    #[test]
    fn test_12() {
        assert_eq!(Some(9), collatz(12));
    }
    #[test]
    fn test_1000000() {
        assert_eq!(Some(152), collatz(1_000_000));
    }
    #[test]
    fn test_0() {
        assert_eq!(None, collatz(0));
    }
    #[test]
    fn test_110243094271() {
        let val = 110243094271;
        assert_eq!(None, collatz(val));
    }
    #[test]
    fn test_max_div_3() {
        let max = u64::MAX / 3;
        assert_eq!(None, collatz(max));
    }
    #[test]
    fn test_max_minus_1() {
        let max = u64::MAX - 1;
        assert_eq!(None, collatz(max));
    }
}
