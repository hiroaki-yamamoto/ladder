use ::std::iter::Iterator;

use crate::entities::Ladder;

#[derive(Debug, Clone)]
pub struct LadderCalcIter {
  delta: f64,
  bpt: f64,
  weight: f64,
  ladder_size: u16,
  cur_ladder: u16,
  cur: Ladder,
}

impl LadderCalcIter {
  pub fn new(
    high: f64,
    low: f64,
    num_ladder: u16,
    bpt: f64,
    weight: Option<f64>,
  ) -> Self {
    // Calculate the distance between each ladder
    let delta = f64::abs(high - low) / (num_ladder - 1) as f64;
    let weight = weight.unwrap_or(1.0);
    let high = if high > low { high } else { low };
    return Self {
      delta,
      bpt,
      weight,
      ladder_size: num_ladder,
      cur_ladder: 0,
      cur: Ladder::new(high, 0),
    };
  }
}

impl Iterator for LadderCalcIter {
  type Item = Ladder;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cur_ladder >= self.ladder_size {
      return None;
    }
    let price = self.cur.price;
    // Unlike cryptos, stocks cannot be bought in fractions
    let qty = ((self.bpt / price) + 0.5) as u64;
    // Budget per trade = BPT * weight^i = previous budget * weight
    self.bpt *= self.weight;
    // Round off the price to 2 decimal places
    let next_price = (((price - self.delta) + 0.005) * 100.0) as u128;
    let next_price = next_price as f64 / 100.0;
    self.cur.price = next_price;
    self.cur.qty = qty;
    self.cur_ladder += 1;
    return Some(Ladder::new(price, qty));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_param_reversed() {
    let svc = LadderCalcIter::new(90.0, 100.0, 5, 1000.0, None);
    let results = &[
      Ladder::new(100.0, 10),
      Ladder::new(97.5, 10),
      Ladder::new(95.0, 11),
      Ladder::new(92.5, 11),
      Ladder::new(90.0, 11),
    ];
    let ladders: Vec<Ladder> = svc.into_iter().collect();
    assert_eq!(ladders, results);
  }

  #[test]
  fn test_calc_without_weight() {
    let svc = LadderCalcIter::new(100.0, 90.0, 5, 1000.0, None);
    let results = &[
      Ladder::new(100.0, 10),
      Ladder::new(97.5, 10),
      Ladder::new(95.0, 11),
      Ladder::new(92.5, 11),
      Ladder::new(90.0, 11),
    ];
    let ladders: Vec<Ladder> = svc.into_iter().collect();
    assert_eq!(ladders, results);
  }

  #[test]
  fn test_calc_with_weight() {
    let svc = LadderCalcIter::new(100.0, 90.0, 5, 1000.0, Some(1.1));
    let results = &[
      Ladder::new(100.0, 10),
      Ladder::new(97.5, 11),
      Ladder::new(95.0, 13),
      Ladder::new(92.5, 14),
      Ladder::new(90.0, 16),
    ];
    let ladders: Vec<Ladder> = svc.into_iter().collect();
    assert_eq!(ladders, results);
  }

  #[test]
  fn test_last_inclusion() {
    let svc = LadderCalcIter::new(22.30, 33.66, 4, 1000.0, None);
    let results = &[
      Ladder::new(33.66, 30),
      Ladder::new(29.87, 33),
      Ladder::new(26.08, 38),
      Ladder::new(22.29, 45),
    ];
    let ladders: Vec<Ladder> = svc.into_iter().collect();
    assert_eq!(ladders, results);
  }

  #[test]
  fn test_rounding_off() {
    let svc = LadderCalcIter::new(19.38, 11.25, 4, 1000.0, Some(1.1));
    let results = &[
      Ladder::new(19.38, 52),
      Ladder::new(16.67, 66),
      Ladder::new(13.96, 87),
      Ladder::new(11.25, 118),
    ];
    let ladders: Vec<Ladder> = svc.into_iter().collect();
    assert_eq!(ladders, results);
  }

  #[test]
  fn test_ladder_length() {
    let svc = LadderCalcIter::new(23.12, 16.18, 4, 1000.0, Some(1.1));
    let results = &[
      Ladder::new(23.12, 43),
      Ladder::new(20.81, 53),
      Ladder::new(18.5, 65),
      Ladder::new(16.19, 82),
    ];
    let ladders: Vec<Ladder> = svc.into_iter().collect();
    assert_eq!(ladders, results);
  }
}
