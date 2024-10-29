use crate::entities::Ladder;

pub trait ILadderCalcSvc {
  /// Calculate the ladder
  fn calc(
    &self,
    high: f64,
    low: f64,
    num_ladder: u16,
    bpt: f64,            // BPT: Base-budget per trade
    weight: Option<f64>, // Re-investment weight: current budget = previous budget * weight
  ) -> Vec<Ladder>;
}
