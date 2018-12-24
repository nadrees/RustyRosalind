use std::io;
use RustyRosalind::{DNAChain, RNAChain};

fn main() {
  println!("Paste input DNA string:");

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let nucleotides = RNAChain::from(DNAChain::parse_nucleotides(&input));
  println!("{}", nucleotides);
}
