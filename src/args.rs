use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct StrandOnlyArgs {
  #[clap()]
  pub strand: String,
}
