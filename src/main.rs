mod cmd;
mod entities;
mod svcs;

use ::clap::Parser;

use crate::cmd::CmdArgs;

fn main() {
  let args = CmdArgs::parse();
}
