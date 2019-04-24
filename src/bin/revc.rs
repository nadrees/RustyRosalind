use rusty_rosalind::nucleotides::dna_chain::DNAChain;
use rusty_rosalind::start_standard_program;

fn main() {
    let dna = DNAChain::parse_str(&start_standard_program!("REVC"));
    print!("{}", dna.reverse_compliment());
}
