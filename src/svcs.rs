use crate::entities::Ladder;
use crate::traits::ILadderCalcSvc;

pub struct LadderCalcSvc {}

impl ILadderCalcSvc for LadderCalcSvc {
  fn calc(
    &self,
    high: f64,
    low: f64,
    num_ladder: u16,
    bpt: f64,
    weight: Option<f64>,
  ) -> Vec<Ladder> {
    // Calculate the distance between each ladder
    let dist = f64::abs(high - low) / (num_ladder - 1) as f64;
    let dist = ((dist + 0.005) * 100.0) as u128;
    let dist = dist as f64 / 100.0;
    let weight = weight.unwrap_or(1.0);
    let mut ladders = Vec::new();
    for i in 0..num_ladder {
      let price = high - dist * i as f64;
      // Budget per trade = BPT * weight^i = previous budget * weight
      let bpt = bpt * f64::powf(weight, i as f64);
      // Unlike cryptos, stocks cannot be bought in fractions
      let qty = ((bpt / price) + 0.5) as u64;
      ladders.push(Ladder::new(price, qty));
    }
    return ladders;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_without_weight() {
    let svc = LadderCalcSvc {};
    let results = &[
      Ladder::new(100.0, 10),
      Ladder::new(97.5, 10),
      Ladder::new(95.0, 11),
      Ladder::new(92.5, 11),
      Ladder::new(90.0, 11),
    ];
    let ladders = svc.calc(100.0, 90.0, 5, 1000.0, None);
    assert_eq!(ladders, results);
  }

  #[test]
  fn test_calc_with_weight() {
    let svc = LadderCalcSvc {};
    let results = &[
      Ladder::new(100.0, 10),
      Ladder::new(97.5, 11),
      Ladder::new(95.0, 13),
      Ladder::new(92.5, 14),
      Ladder::new(90.0, 16),
    ];
    let ladders = svc.calc(100.0, 90.0, 5, 1000.0, Some(1.1));
    assert_eq!(ladders, results);
  }
}
