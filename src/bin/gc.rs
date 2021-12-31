use clap::Parser;
use rusty_rosalind::args::FileArgs as Args;
use rusty_rosalind::fasta::{self, Fasta};
use rusty_rosalind::nucleotides::dna::DNA;

fn main() {
  let args = Args::parse();
  let records: Vec<Fasta<DNA>> = fasta::read_records_from_result_lines(args.read_file().unwrap());

  let mut max_gc_name: Option<String> = None;
  let mut max_gc_value: Option<f32> = None;
  for record in records {
    let gc_content = record.strand.get_gc_content();
    match max_gc_value {
      None => {
        max_gc_value = Some(gc_content);
        max_gc_name = Some(record.name);
      }
      Some(max) if max < gc_content => {
        max_gc_value = Some(gc_content);
        max_gc_name = Some(record.name);
      }
      _ => (),
    }
  }

  println!("{}", max_gc_name.unwrap());
  println!("{}", max_gc_value.unwrap() * 100.0);
}
