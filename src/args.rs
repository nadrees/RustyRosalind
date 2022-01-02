use crate::nucleotides::Nucleotide;
use crate::strands::Strand;
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
pub struct TwoStrandsArgs {
  #[clap()]
  pub strand_1: String,
  #[clap()]
  pub strand_2: String,
}

impl TwoStrandsArgs {
  pub fn parse_strands<T>(
    &self,
  ) -> Result<(Strand<T>, Strand<T>), <Strand<T> as std::str::FromStr>::Err>
  where
    T: Nucleotide,
  {
    let strand_1: Strand<T> = self.strand_1.parse()?;
    let strand_2: Strand<T> = self.strand_2.parse()?;
    Ok((strand_1, strand_2))
  }
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
