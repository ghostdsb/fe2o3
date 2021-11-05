/*
# Bowling

Welcome to Bowling on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Score a bowling game.

Bowling is a game where players roll a heavy ball to knock down pins
arranged in a triangle. Write code to keep track of the score
of a game of bowling.

## Scoring Bowling

The game consists of 10 frames. A frame is composed of one or two ball
throws with 10 pins standing at frame initialization. There are three
cases for the tabulation of a frame.

* An open frame is where a score of less than 10 is recorded for the
  frame. In this case the score for the frame is the number of pins
  knocked down.

* A spare is where all ten pins are knocked down by the second
  throw. The total value of a spare is 10 plus the number of pins
  knocked down in their next throw.

* A strike is where all ten pins are knocked down by the first
  throw. The total value of a strike is 10 plus the number of pins
  knocked down in the next two throws. If a strike is immediately
  followed by a second strike, then the value of the first strike
  cannot be determined until the ball is thrown one more time.

Here is a three frame example:

| Frame 1         | Frame 2       | Frame 3                |
| :-------------: |:-------------:| :---------------------:|
| X (strike)      | 5/ (spare)    | 9 0 (open frame)       |

Frame 1 is (10 + 5 + 5) = 20

Frame 2 is (5 + 5 + 9) = 19

Frame 3 is (9 + 0) = 9

This means the current running total is 48.

The tenth frame in the game is a special case. If someone throws a
strike or a spare then they get a fill ball. Fill balls exist to
calculate the total of the 10th frame. Scoring a strike or spare on
the fill ball does not give the player more fill balls. The total
value of the 10th frame is the total number of pins knocked down.

For a tenth frame of X1/ (strike and a spare), the total value is 20.

For a tenth frame of XXX (three strikes), the total value is 30.

*/
#[derive(Debug, PartialEq)]
pub enum Error {
  NotEnoughPinsLeft,
  GameComplete,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Frame(u8, Option<u16>, Option<u16>, Option<u16>);

impl Frame {
  fn new(index: u8, throw1: Option<u16>, throw2: Option<u16>, throw3: Option<u16>) -> Self {
    Frame(index, throw1, throw2, throw3)
  }
}

#[derive(Debug)]
pub struct BowlingGame {
  pub frames: Vec<Frame>,
}

fn get_score(throw: &Option<u16>) -> u16 {
  match throw {
    Some(score) => *score,
    None => 0,
  }
}

fn is_strike(frame: Frame) -> bool {
  get_score(&frame.1) == 10
}

fn is_spare(frame: Frame) -> bool {
  get_score(&frame.1) != 10 && get_score(&frame.1) + get_score(&frame.2) == 10
}

impl BowlingGame {
  pub fn new() -> Self {
    Self { frames: vec![] }
  }

  pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
    if pins > 10 {
      return Err(Error::NotEnoughPinsLeft);
    }

    let frames = self.frames.clone();
    match frames.last() {
      None => {
        //game starts
        self.frames.push(Frame::new(1, Some(pins), None, None));
      }
      Some(Frame(10, Some(10), Some(10), None)) => {
        // fill ball after consecutive stikes on last frame
        self.frames.pop();
        self
          .frames
          .push(Frame::new(10, Some(10), Some(10), Some(pins)));
      }
      Some(Frame(10, Some(10), Some(p1), None)) => {
        // fill ball
        if p1 + pins > 10 {
          return Err(Error::NotEnoughPinsLeft);
        }
        self.frames.pop();
        self
          .frames
          .push(Frame::new(10, Some(10), Some(*p1), Some(pins)));
      }
      Some(Frame(10, Some(p1), Some(p2), None)) => {
        // last frame
        if p1 + p2 == 10 {
          //  last frame spare
          self.frames.pop();
          self
            .frames
            .push(Frame::new(10, Some(*p1), Some(*p2), Some(pins)))
        } else {
          // last frame; no spare
          return Err(Error::GameComplete);
        }
      }
      Some(Frame(10, Some(10), None, None)) => {
        //last frame's second strike
        self.frames.pop();
        self.frames.push(Frame::new(10, Some(10), Some(pins), None));
      }
      Some(Frame(index, Some(10), None, None)) => {
        // post strike, frame changes
        self
          .frames
          .push(Frame::new(*index + 1, Some(pins), None, None));
      }
      Some(Frame(index, Some(p1), None, None)) => {
        // second throw of a frame
        if p1 + pins > 10 {
          return Err(Error::NotEnoughPinsLeft);
        }
        self.frames.pop();
        self
          .frames
          .push(Frame::new(*index, Some(*p1), Some(pins), None));
      }
      Some(Frame(index, Some(_p1), Some(_p2), None)) => {
        // first throw of new frame
        self
          .frames
          .push(Frame::new(*index + 1, Some(pins), None, None));
      }
      Some(_) => {
        // throw after game complete
        return Err(Error::GameComplete);
      }
    }

    Ok(())
  }

  pub fn score(&self) -> Option<u16> {
    match self.frames.last() {
      Some(Frame(10, Some(10), None, None)) => return None,
      Some(Frame(10, Some(10), Some(10), None)) => return None,
      Some(Frame(10, Some(p1), Some(p2), None)) => {
        if p1 + p2 == 10 {
          return None;
        }
      }
      Some(Frame(10, _, _, _)) => {}
      _ => return None,
    }

    let mut score = 0;

    for frame in self.frames.iter() {
      let Frame(idx, t1, t2, t3) = frame;
      score += get_score(t1) + get_score(t2) + get_score(t3);

      let on_strike = is_strike(*frame);
      let on_spare = is_spare(*frame);

      if on_strike && *idx != 10 {
        score += self.get_next_2_throw_score(*idx as usize);
      }
      if on_spare && *idx != 10 {
        score += self.get_next_throw_score(*idx as usize);
      }
    }

    Some(score)
  }

  fn get_next_2_throw_score(&self, current_frame_index: usize) -> u16 {
    let current_frame = self.frames[current_frame_index];
    let Frame(_, t1, t2, _t3) = current_frame;

    let mut score = get_score(&t1) + get_score(&t2);

    if is_strike(current_frame) && current_frame_index < 9 {
      let Frame(_, t1, _t2, _t3) = self.frames[current_frame_index + 1];
      score += get_score(&t1);
    }

    score
  }

  fn get_next_throw_score(&self, current_frame: usize) -> u16 {
    let Frame(_, t1, _t2, _t3) = self.frames[current_frame];
    get_score(&t1)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn roll_returns_a_result() {
    let mut game = BowlingGame::new();
    assert!(game.roll(0).is_ok());
  }

  #[test]
  fn you_cannot_roll_more_than_ten_pins_in_a_single_roll() {
    let mut game = BowlingGame::new();

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
  }

  #[test]
  fn a_game_score_is_some_if_ten_frames_have_been_rolled() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
      let _ = game.roll(0);
      let _ = game.roll(0);
    }

    assert!(game.score().is_some());
  }

  #[test]
  fn you_cannot_score_a_game_with_no_rolls() {
    let game = BowlingGame::new();

    assert_eq!(game.score(), None);
  }

  #[test]
  fn a_game_score_is_none_if_fewer_than_ten_frames_have_been_rolled() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
      let _ = game.roll(0);
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), None);
  }

  #[test]
  fn a_roll_is_err_if_the_game_is_done() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
      let _ = game.roll(0);
      let _ = game.roll(0);
    }

    assert_eq!(game.roll(0), Err(Error::GameComplete));
  }

  #[test]
  fn twenty_zero_pin_rolls_scores_zero() {
    let mut game = BowlingGame::new();

    for _ in 0..20 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(0));
  }

  #[test]
  fn ten_frames_without_a_strike_or_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
      let _ = game.roll(3);
      let _ = game.roll(6);
    }

    assert_eq!(game.score(), Some(90));
  }

  #[test]
  fn spare_in_the_first_frame_followed_by_zeros() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
  }

  #[test]
  fn points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);
    let _ = game.roll(3);

    for _ in 0..17 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(16));
  }

  #[test]
  fn consecutive_spares_each_get_a_one_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(3);
    let _ = game.roll(7);
    let _ = game.roll(4);

    for _ in 0..15 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(31));
  }

  #[test]
  fn if_the_last_frame_is_a_spare_you_get_one_extra_roll_that_is_scored_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(7);

    assert_eq!(game.score(), Some(17));
  }

  #[test]
  fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
  }

  #[test]
  fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..16 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(26));
  }

  #[test]
  fn consecutive_strikes_each_get_the_two_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..12 {
      let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(81));
  }

  #[test]
  fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(18));
  }

  #[test]
  fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(3);

    assert_eq!(game.score(), Some(20));
  }

  #[test]
  fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(30));
  }

  #[test]
  fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(7);
    let _ = game.roll(3);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(20));
  }

  #[test]
  fn all_strikes_is_a_perfect_score_of_300() {
    let mut game = BowlingGame::new();

    for _ in 0..12 {
      let _ = game.roll(10);
    }

    assert_eq!(game.score(), Some(300));
  }

  #[test]
  fn you_cannot_roll_more_than_ten_pins_in_a_single_frame() {
    let mut game = BowlingGame::new();

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
  }

  #[test]
  fn first_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
  }

  #[test]
  fn the_two_balls_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
  }

  #[test]
  fn the_two_balls_after_a_final_strike_can_be_a_strike_and_non_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert!(game.roll(6).is_ok());
  }

  #[test]
  fn the_two_balls_after_a_final_strike_cannot_be_a_non_strike_followed_by_a_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(6).is_ok());
    assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
  }

  #[test]
  fn second_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins_even_if_first_is_strike(
  ) {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
  }

  #[test]
  fn if_the_last_frame_is_a_strike_you_cannot_score_before_the_extra_rolls_are_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
  }

  #[test]
  fn if_the_last_frame_is_a_spare_you_cannot_create_a_score_before_extra_roll_is_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
      let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
  }

  #[test]
  fn cannot_roll_after_bonus_roll_for_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
      let _ = game.roll(0);
      let _ = game.roll(0);
    }
    let _ = game.roll(7);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
  }

  #[test]
  fn cannot_roll_after_bonus_roll_for_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
      let _ = game.roll(0);
      let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
  }

  #[test]
  fn last_two_strikes_followed_by_only_last_bonus_with_non_strike_points() {
    let mut game = BowlingGame::new();
    for _ in 0..16 {
      let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(0);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(31));
  }
}
