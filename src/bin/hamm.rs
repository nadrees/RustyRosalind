use clap::Parser;
use rusty_rosalind::args::TwoStrandsArgs as Args;
use rusty_rosalind::nucleotides::dna::DNA;

fn main() {
  let args = Args::parse();
  let (strand_1, strand_2) = args.parse_strands::<DNA>().unwrap();
  println!("{}", strand_1.distance(&strand_2));
}
