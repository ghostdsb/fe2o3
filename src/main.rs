mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_26;
mod day_27;
mod day_28;
mod day_29;
mod day_3;
mod day_30;
mod day_31;
mod day_32;
mod day_33;
mod day_34;
mod day_35;
mod day_36;
mod day_37;
mod day_39;
mod day_4;
mod day_40;
mod day_41;
mod day_42;
mod day_43;
mod day_44;
mod day_45;
mod day_49;
mod day_5;
mod day_50;
mod day_51;
mod day_52;
mod day_53;
mod day_54;
mod day_6;
mod day_68;
mod day_7;
mod day_70;
mod day_71;
mod day_72;
mod day_73;
mod day_74;
mod day_75;
mod day_8;
mod day_9;

fn main() {
  println!("Daily Rust Exercise");
  println!("=========");
  println!("=========");
  let input = day_4::is_valid("055 444 285");
  println!("day4: {}", input);

  println!("=========");
  let input = day_5::check("055 444 285");
  println!("day5: {}", input);

  println!("=========");
  let input = day_7::movie(18, 40, 0.47);
  println!("day7: {}", input);

  println!("=========");
  let input = day_8::annotate(&["  *  ", "  *  ", "*****", "  *  ", "  *  "]);
  println!("day8: {:?}", input);

  println!("=========");
  let m = day_9::mean("Rome", &da_ta());
  let v = day_9::variance("Rome", &da_ta());
  println!("day9: {}, {}", m, v);

  println!("=========");
  let x = day_10::frequency(&["aabbcccd", "eefg"], 4);
  println!("day10: {:?}", x);

  println!("=========");
  let hm = hashmap!(1 => "a",);
  println!("day11: {:?}", hm);

  println!("=========");
  let str = day_12::strong(145);
  println!("day12: 145 is {}", str);

  println!("=========");
  let num = day_13::allyourbase::convert(&[1, 2, 0], 2, 10);
  match num {
    Ok(val) => println!("day13: bin 10: {:?}", val),
    Err(error) => println!("day13: err {:?}", error),
  }

  println!("=========");
  let allergy_score = 5;
  let allergion = day_14::allergies::Allergies::new(allergy_score);
  println!("day14: allergy score to {}", allergy_score);
  for (i, item) in day_14::allergies::ALLERGENS.iter().enumerate() {
    let al_score = 2i32.pow(i as u32);
    match allergion.is_allergic_to(item) {
      true => println!("allergic to {:?}({})", item, al_score),
      false => println!("not allergic to {:?}({})", item, al_score),
    }
  }
  println!("{:?}", allergion.allergies());

  println!("=========");
  let ans = day_15::jumping_number(12345);
  println!("{}", ans);

  println!("=========");
  let input = [
    day_16::Direction::North,
    day_16::Direction::South,
    day_16::Direction::South,
    day_16::Direction::East,
    day_16::Direction::West,
    day_16::Direction::North,
    day_16::Direction::West,
  ];
  let dir = day_16::dir_reduc(&input);
  println!("day16: dirs {:?}", dir);

  println!("=========");

  let mut game = day_17::BowlingGame::new();

  let _ = game.roll(10);
  let _ = game.roll(10);
  let _ = game.roll(10);
  let _ = game.roll(5);
  let _ = game.roll(3);

  for _ in 0..12 {
    let _ = game.roll(0);
  }
  println!("{:?}", game);
  println!("{:?}", game.score());

  println!("=========");
  let hd1 = day_19::hamming::hamming_distance("ATGC", "ATGC");
  let hd2 = day_19::hamming::hamming_distance("ATGC", "ATGG");
  let hd3 = day_19::hamming::hamming_distance("ATGC", "ATGGG");

  println!("day19: {:?}, {:?}, {:?}", hd1, hd2, hd3);
  println!("=========");

  let mut s = day_20::grade_school::School::new();
  s.add(3, "Chelsea");
  s.add(7, "Logan");
  println!("day20: {:?}", s.grades());
  println!("day20: {:?}, {:?}", s.grade(3), s.grade(7));

  println!("=========");
  // let isbn = "3-598-21508-8";
  // let isbn = "3-598-21507-X";
  let isbn = "3-598-2X507-9";
  let is_isbn = day_21::is_valid_isbn(isbn);
  println!("day21: {:?}, {:?}", isbn, is_isbn);

  println!("=========");
  let val = "ABBBCCCC  AA";
  let rle_val = day_26::rle::encode(val);
  println!("day26: {:?}, {:?}", val, rle_val);

  println!("=========");
  let val = "D2A8B";
  let rle_val = day_26::rle::decode(val);
  println!("day26: {:?}, {:?}", val, rle_val);

  println!("=========");
  let primes: Vec<u64> = vec![
    5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
  ];
  let private_keys: Vec<u64> = primes
    .iter()
    .map(|x| day_27::diffie_hellman::private_key(*x))
    .collect();

  for i in 0..primes.len() {
    println!("day27: pk: {}, p: {}", private_keys[i], primes[i]);
  }

  println!("=========");
  println!("day_28: {:?}", day_28::sieve::primes_up_to(0));
  println!("day_28: {:?}", day_28::sieve::primes_up_to(2));
  println!("day_28: {:?}", day_28::sieve::primes_up_to(3));
  println!("day_28: {:?}", day_28::sieve::primes_up_to(5));
  println!("day_28: {:?}", day_28::sieve::primes_up_to(11));
  println!("day_28: {:?}", day_28::sieve::primes_up_to(13));

  println!("=========");
  let place = "Varanasi";
  day_31::hello(place);

  println!("=========");
  let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
    + "Devastating Donkeys;Courageous Californians;draw\n"
    + "Devastating Donkeys;Allegoric Alaskans;win\n"
    + "Courageous Californians;Blithering Badgers;loss\n"
    + "Blithering Badgers;Devastating Donkeys;loss\n"
    + "Allegoric Alaskans;Courageous Californians;win";
  let table = day_34::tournament::tally(&input);

  println!("day_34: \n{}", table);

  println!("=========");
  let x: Vec<i32> = day_37::accumulate::map(vec![1, 2, 3], |c: i32| c * c);

  println!("day_37: {:?}", x);

  println!("=========");
  let x = day_39::num_primorial(6_usize);
  println!("{:?}", x);

  println!("=========");
  let _x = day_40::tank_vol(0, 0, 0);

  println!("=========");
  let x = day_45::persistence(4);
  println!("x: {}", x);
  let x = day_45::persistence(39);
  println!("x: {}", x);

  println!("=========");
  let b1 = r#"
1000.00
125 Market 125.45
126 Hardware 34.95
127 Video 7.45
128 Book 14.32
129 Gasoline 16.10
"#;
  let x = day_49::ledger::balance(b1);
  println!("x: \n{}", x);

  println!("=========");
  let x = day_51::simpson(290);
  println!("day_51: {}", x);

  println!("=========");
  let x = day_53::list_squared(1, 250);
  println!("day_53: {:?}", x);

  println!("=========");
  let x = day_54::buddy(10, 50);
  println!("day_53: {:?}", x);

  println!("=========");
  const FIGHTERS: [[&str; 6]; 2] = [
    ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
    ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
  ];

  use day_70::Direction::*;
  let moves = [Left, Left, Left, Left, Left, Left, Left, Left];
  let x = day_70::street_fighter_selection(&FIGHTERS, &[0, 0], &moves);
  println!("day_53: {:?}", x);

  println!("=========");
  let x = day_71::prime_factors(7775460);
  println!("day_71: {:?}", x);

  println!("=========");
  let x = day_74::two_set::solve(7);
  println!("day_74: {:?}", x);

  println!("=========");
  let x = day_75::factorial_trailing_zero::solve(20);
  println!("day_75: {:?}", x);
}

fn da_ta() -> String {
  // don't move the string below

  let dr0 = r#"Rome:Jan 81.2,Feb 63.2,Mar 70.3,Apr 55.7,May 53.0,Jun 36.4,Jul 17.5,Aug 27.5,Sep 60.9,Oct 117.7,Nov 111.0,Dec 97.9
London:Jan 48.0,Feb 38.9,Mar 39.9,Apr 42.2,May 47.3,Jun 52.1,Jul 59.5,Aug 57.2,Sep 55.4,Oct 62.0,Nov 59.0,Dec 52.9
Paris:Jan 182.3,Feb 120.6,Mar 158.1,Apr 204.9,May 323.1,Jun 300.5,Jul 236.8,Aug 192.9,Sep 66.3,Oct 63.3,Nov 83.2,Dec 154.7
NY:Jan 108.7,Feb 101.8,Mar 131.9,Apr 93.5,May 98.8,Jun 93.6,Jul 102.2,Aug 131.8,Sep 92.0,Oct 82.3,Nov 107.8,Dec 94.2
Vancouver:Jan 145.7,Feb 121.4,Mar 102.3,Apr 69.2,May 55.8,Jun 47.1,Jul 31.3,Aug 37.0,Sep 59.6,Oct 116.3,Nov 154.6,Dec 171.5
Sydney:Jan 103.4,Feb 111.0,Mar 131.3,Apr 129.7,May 123.0,Jun 129.2,Jul 102.8,Aug 80.3,Sep 69.3,Oct 82.6,Nov 81.4,Dec 78.2
Bangkok:Jan 10.6,Feb 28.2,Mar 30.7,Apr 71.8,May 189.4,Jun 151.7,Jul 158.2,Aug 187.0,Sep 319.9,Oct 230.8,Nov 57.3,Dec 9.4
Tokyo:Jan 49.9,Feb 71.5,Mar 106.4,Apr 129.2,May 144.0,Jun 176.0,Jul 135.6,Aug 148.5,Sep 216.4,Oct 194.1,Nov 95.6,Dec 54.4
Beijing:Jan 3.9,Feb 4.7,Mar 8.2,Apr 18.4,May 33.0,Jun 78.1,Jul 224.3,Aug 170.0,Sep 58.4,Oct 18.0,Nov 9.3,Dec 2.7
Lima:Jan 1.2,Feb 0.9,Mar 0.7,Apr 0.4,May 0.6,Jun 1.8,Jul 4.4,Aug 3.1,Sep 3.3,Oct 1.7,Nov 0.5,Dec 0.7
"#;
  String::from(dr0)
}
