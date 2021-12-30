use clap::Parser;
use rusty_rosalind::nucleotides::{DNA, RNA};
use rusty_rosalind::strands::Strand;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap()]
  dna_string: String,
}

fn main() {
  let args = Args::parse();
  let dna_string: Strand<DNA> = args.dna_string.parse().unwrap();
  let rna_string: Strand<RNA> = dna_string.transcribe();
  println!("{}", rna_string);
}
