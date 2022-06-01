use clap::Parser;
use rusty_rosalind::args::FileArgs as Args;
use rusty_rosalind::fasta::{self, Fasta};
use rusty_rosalind::nucleotides::dna::DNA;

fn main() {
  let args = Args::parse();
  let records: Vec<Fasta<DNA>> = fasta::read_records_from_result_lines(args.read_file().unwrap());

  for i in 0..records.len() {
    let current = &records[i];
    for j in 0..records.len() {
      if i == j {
        continue;
      }
      let other = &records[j];
      if current.strand.overlaps_with_length(&other.strand, 3) {
        println!("{} {}", current.name, other.name);
      }
    }
  }
}
