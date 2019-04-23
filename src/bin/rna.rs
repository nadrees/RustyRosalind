use clap::{App, Arg};
use rusty_rosalind::nucleotides::{dna_chain::DNAChain, rna_chain::RNAChain};
use std::fs;

fn main() {
    let matches = App::new("RNA")
        .arg(Arg::with_name("input_file").required(true).index(1))
        .get_matches();
    let f_name = matches.value_of("input_file").unwrap();
    let dna = DNAChain::parse_str(fs::read_to_string(f_name).unwrap().as_ref());
    let rna = RNAChain::from(dna);
    print!("{}", rna);
}
