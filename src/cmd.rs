use ::clap::Parser;

/// Calculate the ladder price and qty to buy for stock trading (aka. DCA calculator).
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
  /// The number of ladders to calculate.
  #[arg(short, long, default_value_t = 4)]
  pub num_ladder: u16,

  /// The base budget per trade.
  #[arg(short, long, default_value_t = 1000.0)]
  pub bese_budget_per_trade: f64,

  /// The weight to increase / decrease the budget per trade.
  /// The buget when trading is calculated as bpt = previous bpt * weight.
  #[arg(short, long, default_value = "1.0")]
  pub weight: f64,

  /// The minimum price to end the ladder.
  #[arg(short, long)]
  pub low: f64,

  /// The maximum price to start the ladder.
  #[arg(short = 'm', long)]
  pub high: f64,
}
