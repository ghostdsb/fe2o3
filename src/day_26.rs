pub mod rle {
    pub fn encode(source: &str) -> String {
        let mut stack: Vec<(i32, char)> = vec![];

        for chr in source.chars() {
            match stack.last_mut() {
                Some((count, val)) => {
                    if *val == chr {
                        *count += 1;
                    } else {
                        stack.push((1, chr))
                    }
                }
                None => stack.push((1, chr)),
            }
        }

        println!("{:?}", stack);

        stack.iter().fold(String::default(), |acc, (count, chr)| {
            if count > &1_i32 {
                format!("{}{}{}", acc, count, chr)
            } else {
                format!("{}{}", acc, chr)
            }
        })
    }

    pub fn decode(source: &str) -> String {
        let make_dups = |count: i32, chr: &str| {
            if count == 0 {
                return String::from(chr);
            }
            (0..count).fold(String::default(), |mut acc, _| {
                acc.push_str(chr);
                acc
            })
        };

        let mut bin = (String::default(), String::default());
        for chr in source.chars() {
            if chr.is_digit(10) {
                bin.0 += &chr.to_string();
            } else {
                let count = match bin.0.parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => 0,
                };
                bin.0 = String::default();
                bin.1 += &make_dups(count, &chr.to_string())
            }
        }
        bin.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // encoding tests
    #[test]
    fn test_encode_empty_string() {
        assert_eq!("", rle::encode(""));
    }
    #[test]
    fn test_encode_single_characters() {
        assert_eq!("XYZ", rle::encode("XYZ"));
    }
    #[test]
    fn test_encode_string_with_no_single_characters() {
        assert_eq!("2A3B4C", rle::encode("AABBBCCCC"));
    }
    #[test]
    fn test_encode_single_characters_mixed_with_repeated_characters() {
        assert_eq!(
            "12WB12W3B24WB",
            rle::encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB")
        );
    }
    #[test]
    fn test_encode_multiple_whitespace_mixed_in_string() {
        assert_eq!("2 hs2q q2w2 ", rle::encode("  hsqq qww  "));
    }
    #[test]
    fn test_encode_lowercase_characters() {
        assert_eq!("2a3b4c", rle::encode("aabbbcccc"));
    }
    // decoding tests
    #[test]
    fn test_decode_empty_string() {
        assert_eq!("", rle::decode(""));
    }
    #[test]
    fn test_decode_single_characters_only() {
        assert_eq!("XYZ", rle::decode("XYZ"));
    }
    #[test]
    fn test_decode_string_with_no_single_characters() {
        assert_eq!("AABBBCCCC", rle::decode("2A3B4C"));
    }
    #[test]
    fn test_decode_single_characters_with_repeated_characters() {
        assert_eq!(
            "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB",
            rle::decode("12WB12W3B24WB")
        );
    }
    #[test]
    fn test_decode_multiple_whitespace_mixed_in_string() {
        assert_eq!("  hsqq qww  ", rle::decode("2 hs2q q2w2 "));
    }
    #[test]
    fn test_decode_lower_case_string() {
        assert_eq!("aabbbcccc", rle::decode("2a3b4c"));
    }
    // consistency test
    #[test]
    fn test_consistency() {
        assert_eq!(
            "zzz ZZ  zZ",
            rle::decode(rle::encode("zzz ZZ  zZ").as_str())
        );
    }
}
