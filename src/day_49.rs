/**
 * You are given a (small) check book as a - sometimes - cluttered (by non-alphanumeric characters) string:
 * "1000.00
 * 125 Market 125.45
 * 126 Hardware 34.95
 * 127 Video 7.45
 * 128 Book 14.32
 * 129 Gasoline 16.10"
 * The first line shows the original balance.
 * Each other line (when not blank) gives information: check number, category, check amount. (Input form may change depending on the language).
 *
 * "Original_Balance:_1000.00
 * 125_Market_125.45_Balance_874.55
 * 126_Hardware_34.95_Balance_839.60
 * 127_Video_7.45_Balance_832.15
 * 128_Book_14.32_Balance_817.83
 * 129_Gasoline_16.10_Balance_801.73
 * Total_expense__198.27
 * Average_expense__39.65"
 */

pub mod ledger {
  pub fn balance(book: &str) -> String {
    let mut led = String::new();
    let mut initial = 0.0;
    let mut bal = 0.0;
    let mut items = 0;
    book.split('\n').for_each(|line| {
      let mut record = line.split(" ");
      println!("{:?}", line);
      match (record.next(), record.next(), record.next()) {
        (Some(""), None, None) => {
          let total_expense = initial - bal;
          let average_expense = total_expense / (items as f32);
          led = format!(
            "{}\nTotal expense  {:.2}\nAverage expense  {:.2}",
            led, total_expense, average_expense
          );
        }
        (Some(base), None, None) => {
          bal = base.parse::<f32>().unwrap();
          initial = bal;
          led = format!("Original Balance: {}", base);
        }
        (Some(idx), Some(item), Some(cost)) => {
          let item_cost = cost.parse::<f32>().unwrap();
          bal = bal - item_cost;
          items += 1;
          led = format!(
            "{}\n{} {} {:.2} Balance {:.2}",
            led, idx, item, item_cost, bal
          );
        }
        _ => {}
      }
    });
    led
  }
}

#[cfg(test)]

mod tests {
  use super::ledger::*;
  fn dotest(book: &str, exp: &str) -> () {
    println!("book:{}", book);
    let ans = balance(book);
    println!("actual:\n{}", ans);
    println!("expect:\n{}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
  }

  #[test]
  fn basic_tests() {
    let b1 = r#"
1000.00
125 Market 125.45
126 Hardware 34.95
127 Video 7.45
128 Book 14.32
129 Gasoline 16.10
"#;
    let b2 = r#"
1233.00
125 Hardware 24.80
123 Flowers 93.50
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos 71.40
122 Picture 93.50
132 Tyres 19.00
129 Stamps 13.60
129 Fruits 17.60
129 Market 128.00
121 Gasoline 13.60
"#;

    let b1sol="Original Balance: 1000.00\n125 Market 125.45 Balance 874.55\n126 Hardware 34.95 Balance 839.60\n127 Video 7.45 Balance 832.15\n128 Book 14.32 Balance 817.83\n129 Gasoline 16.10 Balance 801.73\nTotal expense  198.27\nAverage expense  39.65";
    let b2sol="Original Balance: 1233.00\n125 Hardware 24.80 Balance 1208.20\n123 Flowers 93.50 Balance 1114.70\n127 Meat 120.90 Balance 993.80\n120 Picture 34.00 Balance 959.80\n124 Gasoline 11.00 Balance 948.80\n123 Photos 71.40 Balance 877.40\n122 Picture 93.50 Balance 783.90\n132 Tyres 19.00 Balance 764.90\n129 Stamps 13.60 Balance 751.30\n129 Fruits 17.60 Balance 733.70\n129 Market 128.00 Balance 605.70\n121 Gasoline 13.60 Balance 592.10\nTotal expense  640.90\nAverage expense  53.41";

    dotest(b1, b1sol);
    dotest(b2, b2sol);
  }
}
