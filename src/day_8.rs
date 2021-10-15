pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mined: Vec<String> = Vec::with_capacity(minefield.len());

    let minefield_height = minefield.len();

    if minefield_height == 0 {
        return vec![];
    }

    let minefield_width = minefield[0].len();

    if minefield_width == 0 {
        return vec![String::from("")];
    }

    for i in 0..minefield_height {
        for j in 0..minefield_width {
            if get_val(minefield[i].chars().nth(j)) == 1 {
                mined.push(String::from("*"));
            } else {
                let mines = if j > 0 {
                    get_val(minefield[i].chars().nth(j - 1))
                        + if i < minefield_height - 1 {
                            get_val(minefield[i + 1].chars().nth(j - 1))
                        } else {
                            0
                        }
                } else {
                    0
                } + get_val(minefield[i].chars().nth(j + 1))
                    + if i < minefield_height - 1 {
                        get_val(minefield[i + 1].chars().nth(j))
                            + get_val(minefield[i + 1].chars().nth(j + 1))
                    } else {
                        0
                    }
                    + if i > 0 && j > 0 {
                        get_val(minefield[i - 1].chars().nth(j - 1))
                    } else {
                        0
                    }
                    + if i > 0 {
                        get_val(minefield[i - 1].chars().nth(j))
                            + get_val(minefield[i - 1].chars().nth(j + 1))
                    } else {
                        0
                    };
                if mines == 0 {
                    mined.push(String::from(" "));
                } else {
                    mined.push(format!("{}", mines))
                }
            }
        }
    }

    mined
        .chunks(minefield_width)
        .map(|x| x.iter().cloned().collect())
        .collect()
}

fn get_val(val: Option<char>) -> u32 {
    match val {
        Some('*') => 1,
        Some(_) => 0,
        None => 0,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }
    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',
                _ => ' ',
            })
            .collect()
    }
    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);
        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, annotate(&cleaned_strs));
    }
    #[test]
    fn no_rows() {
        #[rustfmt::skip]
        run_test(&[
        ]);
    }
    #[test]
    fn no_columns() {
        #[rustfmt::skip]
        run_test(&[
            "",
        ]);
    }
    #[test]
    fn no_mines() {
        #[rustfmt::skip]
        run_test(&[
            "   ",
            "   ",
            "   ",
        ]);
    }
    #[test]
    fn board_with_only_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "***",
            "***",
        ]);
    }
    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        run_test(&[
            "111",
            "1*1",
            "111",
        ]);
    }
    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        run_test(&[
            "***",
            "*8*",
            "***",
        ]);
    }
    #[test]
    fn horizontal_line() {
        #[rustfmt::skip]
        run_test(&[
            "1*2*1",
        ]);
    }
    #[test]
    fn horizontal_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*1 1*",
        ]);
    }
    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
        run_test(&[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
    }
    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
        run_test(&[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
    }
    #[test]
    fn cross() {
        #[rustfmt::skip]
        run_test(&[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
    }
    #[test]
    fn large_board() {
        #[rustfmt::skip]
        run_test(&[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
    }
}
