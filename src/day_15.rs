/**
 * numbers with difference of consecutive digits equal to 1 are called jumping numbers
 */

pub fn jumping_number(n: u64) -> bool {
    let n = n.to_string().chars().collect::<Vec<char>>();

    if n.len() == 1 {
        return true;
    }

    for i in 0..n.len() - 1 {
        if (n[i].to_digit(10).unwrap() as i32 - n[i + 1].to_digit(10).unwrap() as i32).abs() != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(jumping_number(1), true);
        assert_eq!(jumping_number(7), true);
        assert_eq!(jumping_number(9), true);
        assert_eq!(jumping_number(23), true);
        assert_eq!(jumping_number(32), true);
        assert_eq!(jumping_number(79), false);
        assert_eq!(jumping_number(790), false);
        assert_eq!(jumping_number(98), true);
        assert_eq!(jumping_number(8987), true);
        assert_eq!(jumping_number(4343456), true);
        assert_eq!(jumping_number(98789876), true);
    }
}
