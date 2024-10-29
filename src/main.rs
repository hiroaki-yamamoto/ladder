mod cmd;
mod entities;
mod iter;

use ::clap::Parser;

use crate::cmd::CmdArgs;
use crate::iter::LadderCalcIter;

fn main() {
  let args = CmdArgs::parse();
  let calc = LadderCalcIter::new(
    args.high,
    args.low,
    args.num_ladder,
    args.bese_budget_per_trade,
    Some(args.weight),
  );
  for ladder in calc {
    println!("{}", ladder.to_string());
  }
}
