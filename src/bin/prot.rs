use clap::Parser;
use rusty_rosalind::args::StrandOnlyArgs as Args;
use rusty_rosalind::nucleotides::protein::AminoAcid;
use rusty_rosalind::nucleotides::rna::RNA;
use rusty_rosalind::strands::Strand;

fn main() {
  let args = Args::parse();
  let strand: Strand<RNA> = args.strand.parse().unwrap();
  let protein: Strand<AminoAcid> = Strand::from(strand);
  println!("{}", protein);
}
