use std::io;
use RustyRosalind::{parse_nucleotides, DNA};
use std::collections::HashMap;

fn main() {
  println!("Paste input DNA string:");

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input.trim();

  let nucleotides = parse_nucleotides(&input.trim());
  let mut counts: HashMap<DNA, u32> = HashMap::new();

  for nucleotide in nucleotides {
    let count = counts.entry(nucleotide).or_insert(0);
    *count += 1;
  }

  println!("{} {} {} {}", 
    counts[&DNA::A], 
    counts[&DNA::C], 
    counts[&DNA::G], 
    counts[&DNA::T]
  );
}
