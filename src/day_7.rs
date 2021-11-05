/**
 * My friend John likes to go to the cinema. He can choose between system A and system B.

System A : he buys a ticket (15 dollars) every time
System B : he buys a card (500 dollars) and a first ticket for 0.90 times the ticket price,
then for each additional ticket he pays 0.90 times the price paid for the previous ticket.
Example:
If John goes to the cinema 3 times:

System A : 15 * 3 = 45
System B : 500 + 15 * 0.90 + (15 * 0.90) * 0.90 + (15 * 0.90 * 0.90) * 0.90 ( = 536.5849999999999, no rounding for each ticket)
John wants to know how many times he must go to the cinema so that the final result of System B,
when rounded up to the next dollar, will be cheaper than System A.
 */

pub fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
  let mut system_b = card as f64;
  for day in 1.. {
    let system_a = (ticket * day) as f64;
    system_b += ticket as f64 * f64::powf(perc, day as f64);
    if system_a > system_b.ceil() {
      return day;
    }
  }
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
    let ans = movie(card, ticket, perc);
    assert_eq!(ans, exp);
  }

  #[test]
  fn basic_tests() {
    dotest(500, 15, 0.9, 43);
    dotest(100, 10, 0.95, 24);
    dotest(0, 10, 0.95, 2);
  }
  #[test]
  fn extended_tests() {
    dotest(1114, 15, 0.63, 77);
  }
  #[test]
  fn ceil_tests() {
    dotest(2500, 20, 0.9, 135);
  }

  #[test]
  fn ceil_tests_2() {
    dotest(18, 40, 0.47, 1);
  }
}
