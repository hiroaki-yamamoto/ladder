use ::std::iter::Iterator;

use crate::entities::Ladder;

pub struct LadderCalcSvc {
  low: f64,
  delta: f64,
  bpt: f64,
  weight: f64,
  cur: Ladder,
}

impl LadderCalcSvc {
  pub fn new(
    high: f64,
    low: f64,
    num_ladder: u16,
    bpt: f64,
    weight: Option<f64>,
  ) -> Self {
    // Calculate the distance between each ladder
    let delta = f64::abs(high - low) / (num_ladder - 1) as f64;
    let delta = ((delta + 0.005) * 100.0) as u128;
    let delta = delta as f64 / 100.0;
    let weight = weight.unwrap_or(1.0);
    return Self {
      low,
      delta,
      bpt,
      weight,
      cur: Ladder::new(high, 0),
    };
  }
}

impl Iterator for LadderCalcSvc {
  type Item = Ladder;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cur.price < self.low {
      return None;
    }
    let price = self.cur.price;
    // Unlike cryptos, stocks cannot be bought in fractions
    let qty = ((self.bpt / price) + 0.5) as u64;
    // Budget per trade = BPT * weight^i = previous budget * weight
    self.bpt *= self.weight;
    self.cur = Ladder::new(price - self.delta, qty);
    return Some(Ladder::new(price, qty));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_without_weight() {
    let svc = LadderCalcSvc::new(100.0, 90.0, 5, 1000.0, None);
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
    let svc = LadderCalcSvc::new(100.0, 90.0, 5, 1000.0, Some(1.1));
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
}
