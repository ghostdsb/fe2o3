/*
Time to win the lottery!

Given a lottery ticket (ticket), represented by an array of 2-value arrays, you must find out if you've won the jackpot.

Example ticket:

[ ( "ABC", 65 ), ( "HGR", 74 ), ( "BYHT", 74 ) ]

To do this, you must first count the 'mini-wins' on your ticket. Each subarray has both a string and a number within it.
If the character code of any of the characters in the string matches the number, you get a mini win. Note you can only have one mini win per sub array.

Once you have counted all of your mini wins, compare that number to the other input provided (win).
If your total is more than or equal to (win), return 'Winner!'. Else return 'Loser!'.

All inputs will be in the correct format. Strings on tickets are not always the same length.
*/
#[allow(unused)]
pub fn bingo<S: AsRef<str> + std::fmt::Debug>(ticket: &[(S, u8)], win: usize) -> &'static str {
  if ticket
    .iter()
    .filter(|(letters, number)| letters.as_ref().contains(*number as char))
    .count()
    >= win
  {
    "Winner!"
  } else {
    "Loser!"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    assert_eq!(
      bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2),
      "Loser!"
    );
    assert_eq!(
      bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1),
      "Winner!"
    );
    assert_eq!(
      bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3),
      "Loser!"
    );
    assert_eq!(
      bingo(&[("URA", 89), ("OXXXO", 88), ("VID", 68)], 1),
      "Winner!"
    );
  }
}
