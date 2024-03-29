use clap::Parser;
use rusty_rosalind::args::TwoStrandsArgs as Args;
use rusty_rosalind::nucleotides::dna::DNA;

fn main() {
  let args = Args::parse();
  let (strand_1, strand_2) = args.parse_strands::<DNA>().unwrap();
  let indexes = strand_2
    .substrings(&strand_1)
    .into_iter()
    .map(|i| i + 1)
    .map(|i| i.to_string())
    .collect::<Vec<String>>();
  println!("{}", indexes.join(" "));
}
