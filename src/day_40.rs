/**
 *
 * cos t = (R-h)/R
 * t = acos(R-h)/R
 * a = 2(t)
 *
 * base area BA = pi*r^2*(a/2pi)
 * v = BA * h
 *
 * To introduce the problem think to my neighbor who drives a tanker truck. The level indicator is down and he is worried because he does not know if he will be able to make deliveries. We put the truck on a horizontal ground and measured the height of the liquid in the tank.
 * Fortunately the tank is a perfect cylinder and the vertical walls on each end are flat. The height of the remaining liquid is h, the diameter of the cylinder base is d, the total volume is vt (h, d, vt are positive or null integers). You can assume that h <= d.
 * Could you calculate the remaining volume of the liquid? Your function tankvol(h, d, vt) returns an integer which is the truncated result (e.g floor) of your float calculation.

*/
pub const PI: f64 = std::f64::consts::PI;

pub fn tank_vol(h: i32, d: i32, vt: i32) -> i32 {
  if h == d {
    return vt;
  }

  let h = h as f64;
  let radius = d as f64 / 2.0;
  let length = vt as f64 / (PI * radius.powi(2));

  let area_sector = ((radius - h) / radius).acos() * radius.powi(2);
  let area_triangle = (radius - h) * (2.0 * radius * h - h.powi(2)).sqrt();

  ((area_sector - area_triangle) * length) as i32
}

#[cfg(test)]
mod test {
  use super::*;

  fn dotest(h: i32, d: i32, vt: i32, exp: i32) -> () {
    assert_eq!(tank_vol(h, d, vt), exp)
  }
  #[test]
  fn basics_tank_vol() {
    dotest(5, 7, 3848, 2940);
    dotest(5, 7, 3848, 2940);
    dotest(2, 7, 3848, 907);
    dotest(2, 8, 5026, 982);
    dotest(4, 9, 6361, 2731);
  }
}
