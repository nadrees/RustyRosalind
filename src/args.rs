use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[clap()]
pub struct StrandOnlyArgs {
  #[clap()]
  pub strand: String,
}

#[derive(Parser, Debug)]
#[clap()]
pub struct FileArgs {
  #[clap()]
  pub filename: String,
}

impl FileArgs {
  pub fn read_file(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&self.filename)?;
    Ok(io::BufReader::new(file).lines())
  }
}
