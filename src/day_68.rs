/**
 *
 * Braking distance d1 is the distance a vehicle will go from the point when it brakes to when it comes to a complete stop.
 * It depends on the original speed v and on the coefficient of friction mu between the tires and the road surface.
 * The braking distance is one of two principal components of the total stopping distance.
 * The other component is the reaction distance, which is the product of the speed and the perception-reaction time of the driver.
 * The kinetic energy E is 0.5*m*v**2, the work W given by braking is mu*m*g*d1.
 * Equalling E and W gives the braking distance: d1 = v*v / 2*mu*g where g is the gravity of Earth and m the vehicle's mass.
 *
 * We have v in km per hour, g as 9.81 m/s/s and in the following we suppose that the reaction time is constant and equal to 1 s.
 * The coefficient mu is dimensionless
 *
 */
#[allow(unused)]
pub fn dist(v: f64, mu: f64) -> f64 {
  let v = v / 3.6;
  v + v * v / (2.0 * mu * 9.81)
}

#[allow(unused)]
pub fn speed(d: f64, mu: f64) -> f64 {
  let mu_g = mu * 9.81;
  3.6 * ((mu_g * mu_g + 2.0 * mu_g * d).sqrt() - mu_g)
}

#[cfg(test)]
mod tests {
  use super::{dist, speed};
  use float_eq::float_eq;

  #[test]
  fn basic_tests_dist() {
    assert_float_equals(dist(144.0, 0.3), 311.83146449201496);
    assert_float_equals(dist(92.0, 0.5), 92.12909477605366);
  }

  #[test]
  fn basic_tests_speed() {
    assert_float_equals(speed(159.0, 0.8), 153.79671564846308);
    assert_float_equals(speed(164.0, 0.7), 147.91115701756493);
  }

  fn assert_float_equals(actual: f64, expected: f64) {
    let merr = 1.0e-12;
    let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
    assert!(
      res,
      "Expected value must be near: {:e} but was:{:e}",
      expected, actual
    );
  }
}
