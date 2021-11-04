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
