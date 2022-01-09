use clap::Parser;
use rusty_rosalind::args::StrandOnlyArgs as Args;
use rusty_rosalind::nucleotides::protein::AminoAcid;
use rusty_rosalind::nucleotides::rna::RNA;
use rusty_rosalind::strands::Strand;

fn main() {
  let args = Args::parse();
  let mut protein: Strand<AminoAcid> = args.strand.parse().unwrap();
  protein.push(AminoAcid::Stop);
  let mut i: u128 = 1;
  for amino_acid in protein {
    i *= RNA::reverse_translations(&amino_acid).len() as u128;
    while i >= 1000000 {
      i -= 1000000;
    }
  }
  println!("{}", i);
}
