use clap::Parser;
use rusty_rosalind::nucleotides::dna::DNA;
use rusty_rosalind::strands::Strand;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap()]
  strand_1: String,
  #[clap()]
  strand_2: String,
}

fn main() {
  let args = Args::parse();
  let strand_1: Strand<DNA> = args.strand_1.parse().unwrap();
  let strand_2: Strand<DNA> = args.strand_2.parse().unwrap();
  let indexes = strand_2
    .substrings(&strand_1)
    .into_iter()
    .map(|i| i + 1)
    .map(|i| i.to_string())
    .collect::<Vec<String>>();
  println!("{}", indexes.join(" "));
}
