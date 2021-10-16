/*
Instructions

Add the mine counts to a completed Minesweeper board.
Minesweeper is a popular game where the user has to find the mines using numeric hints that indicate how many mines are directly adjacent (horizontally, vertically, diagonally) to a square.
In this exercise you have to create some code that counts the number of mines adjacent to a given empty square and replaces that square with the count.
The board is a rectangle composed of blank space (' ') characters. A mine is represented by an asterisk ('*') character.
If a given space has no adjacent mines at all, leave that square blank.

Examples
For example you may receive a 5 x 4 board like this (empty spaces are represented here with the '·' character for display on screen):

·*·*·
··*··
··*··
·····
And your code will transform it into this:

1*3*1
13*31
·2*2·
·111·

*/

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mined: Vec<String> = Vec::new();
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
            let r = i as i32;
            let c = j as i32;
            if get_position_val(r, c, minefield_height, minefield_width, minefield) == 1 {
                mined.push(String::from("*"));
            } else {
                let mines =
                    get_position_val(r - 1, c - 1, minefield_height, minefield_width, minefield)
                        + get_position_val(r - 1, c, minefield_height, minefield_width, minefield)
                        + get_position_val(
                            r - 1,
                            c + 1,
                            minefield_height,
                            minefield_width,
                            minefield,
                        )
                        + get_position_val(r, c - 1, minefield_height, minefield_width, minefield)
                        + get_position_val(r, c + 1, minefield_height, minefield_width, minefield)
                        + get_position_val(
                            r + 1,
                            c - 1,
                            minefield_height,
                            minefield_width,
                            minefield,
                        )
                        + get_position_val(r + 1, c, minefield_height, minefield_width, minefield)
                        + get_position_val(
                            r + 1,
                            c + 1,
                            minefield_height,
                            minefield_width,
                            minefield,
                        );
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

fn get_position_val(i: i32, j: i32, w: usize, h: usize, field: &[&str]) -> u32 {
    if i < 0 || j < 0 || i as usize >= w || j as usize >= h {
        0
    } else {
        match field[i as usize].chars().nth(j as usize) {
            Some('*') => 1,
            Some(_) => 0,
            None => 0,
        }
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
