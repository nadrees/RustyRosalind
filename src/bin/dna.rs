use clap::Parser;
use rusty_rosalind::args::StrandOnlyArgs as Args;
use rusty_rosalind::nucleotides::dna::DNA;
use rusty_rosalind::strands::Strand;

fn main() {
    let args = Args::parse();
    let dna_string: Strand<DNA> = args.strand.parse().unwrap();
    let result = dna_string.count_nucleotides();
    println!(
        "{:?} {:?} {:?} {:?}",
        result.get(&DNA::A).unwrap(),
        result.get(&DNA::C).unwrap(),
        result.get(&DNA::G).unwrap(),
        result.get(&DNA::T).unwrap()
    )
}
