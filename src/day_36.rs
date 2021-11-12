pub mod phone_number {

  pub fn number(user_number: &str) -> Option<String> {
    let mut phn: String = user_number.chars().filter(|n| n.is_digit(10)).collect();

    match (
      phn.len(),
      phn.get(0..1),
      phn.get(1..2),
      phn.get(3..4),
      phn.get(4..5),
    ) {
      (10, Some(d1), _, Some(d2), _)
        if d1.parse::<i32>().unwrap() >= 2 && d2.parse::<i32>().unwrap() >= 2 =>
      {
        Some(phn)
      }
      (11, Some("1"), Some(d1), _, Some(d2))
        if d1.parse::<i32>().unwrap() >= 2 && d2.parse::<i32>().unwrap() >= 2 =>
      {
        phn.remove(0);
        Some(phn)
      }
      _ => None,
    }
  }
}
