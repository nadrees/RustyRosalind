use rusty_rosalind::nucleotides::{dna_chain::DNAChain, rna_chain::RNAChain};
use rusty_rosalind::start_standard_program;

fn main() {
    let dna = DNAChain::parse_str(&start_standard_program!("RNA"));
    let rna = RNAChain::from(dna);
    print!("{}", rna);
}
