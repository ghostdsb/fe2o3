/**
 * Instructions
 * Determine if a number is perfect, abundant, or deficient based on Nicomachus' (60 - 120 CE) classification scheme for positive integers.
 * The Greek mathematician Nicomachus devised a classification scheme for positive integers,
 * identifying each as belonging uniquely to the categories of perfect, abundant, or deficient based on their aliquot sum.
 * The aliquot sum is defined as the sum of the factors of a number not including the number itself.
 * For example, the aliquot sum of 15 is (1 + 3 + 5) = 9
 *
 * Perfect: aliquot sum = number
 * 6 is a perfect number because (1 + 2 + 3) = 6
 * 28 is a perfect number because (1 + 2 + 4 + 7 + 14) = 28
 *
 * Abundant: aliquot sum > number
 * 12 is an abundant number because (1 + 2 + 3 + 4 + 6) = 16
 * 24 is an abundant number because (1 + 2 + 3 + 4 + 6 + 8 + 12) = 36
 *
 * Deficient: aliquot sum < number
 * 8 is a deficient number because (1 + 2 + 4) = 7
 * Prime numbers are deficient
 */

#[allow(unused)]
pub mod perfect_numbers {
    use std::cmp::Ordering;
    #[derive(Debug, PartialEq, Eq)]
    pub enum Classification {
        Abundant,
        Perfect,
        Deficient,
    }
    pub fn classify(num: u64) -> Option<Classification> {
        match (
            num,
            (1..num).filter(|n| num % n == 0).sum::<u64>().cmp(&num),
        ) {
            (0, _) => None,
            (_, Ordering::Less) => Some(Classification::Deficient),
            (_, Ordering::Equal) => Some(Classification::Perfect),
            (_, Ordering::Greater) => Some(Classification::Abundant),
        }
    }
}

#[cfg(test)]
mod test {
    use super::perfect_numbers::{classify, Classification};
    macro_rules! tests {
    ($property_test_func:ident {
        $( $(#[$attr:meta])* $test_name:ident( $( $param:expr ),* ); )+
    }) => {
        $(
            $(#[$attr])*
            #[test]
            fn $test_name() {
                $property_test_func($( $param ),* )
            }
        )+
    }
}
    fn test_classification(num: u64, result: Classification) {
        assert_eq!(classify(num), Some(result));
    }
    #[test]
    fn basic() {
        assert_eq!(classify(0), None);
    }
    tests! {
        test_classification {
            test_1(1, Classification::Deficient);
            test_2(2, Classification::Deficient);
            test_4(4, Classification::Deficient);
            test_6(6, Classification::Perfect);
            test_12(12, Classification::Abundant);
            test_28(28, Classification::Perfect);
            test_30(30, Classification::Abundant);
            test_32(32, Classification::Deficient);
            test_33550335(33_550_335, Classification::Abundant);
            test_33550336(33_550_336, Classification::Perfect);
            test_33550337(33_550_337, Classification::Deficient);
        }
    }
}
