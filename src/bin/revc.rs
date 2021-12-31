use clap::Parser;
use rusty_rosalind::args::StrandOnlyArgs as Args;
use rusty_rosalind::nucleotides::dna::DNA;
use rusty_rosalind::strands::Strand;

fn main() {
  let args = Args::parse();
  let dna_string: Strand<DNA> = args.strand.parse().unwrap();
  println!("{}", dna_string.reverse_compliment());
}
