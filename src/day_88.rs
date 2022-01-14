/**
 * Moving stuff to outer space is super expensive and takes a lot of energy,
 * which is part of the reason why colonizing the moon or terraforming Mars is extremely hard.
 *
 * To move something heavy into space you need a rocket with enough fuel.
 * But adding fuel makes the rocket even heavier... And if you wanted to visit Mars and come back, you would need enough fuel to leave both Earth and Mars.
 *
 * We can actually calculate how much fuel a rocket needs using the rocket equation:  
 * m_fuel = M(e^(v/v_e) - 1)
 * where
 * M is the mass of the rocket (with no fuel),  
 * v_e is the exhaust velocity of the rocket, and
 * e is Euler's number.
 * v is the velocity the rocket needs to escape, which is different for every planet.
 *
 * Try to submit some code with a function rocket_fuel(v) that returns  for Saturn V (M = 250000kg, v_e=2550) as a function of v.
 */

pub fn rocket_fuel(v: f32) -> f32 {
  const M: u32 = 250_000;
  let v_e = 2550;
  let e = std::f32::consts::E;
  let m_fuel = M as f32 * (e.powf(v as f32 / v_e as f32) - 1_f32);
  m_fuel
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn earth() {
    assert_eq!(rocket_fuel(11186.0), 19843016.0);
  }
  #[test]
  fn moon() {
    assert_eq!(rocket_fuel(2380.0), 385742.9);
  }
  #[test]
  fn jupiter() {
    assert_eq!(rocket_fuel(60200.0), 4473998400000000.0);
  }
  #[test]
  fn pluto() {
    assert_eq!(rocket_fuel(1230.0), 154970.34);
  }
  #[test]
  fn phobos() {
    assert_eq!(rocket_fuel(1.139), 111.699104);
  }
}
