use clap::Parser;
use rusty_rosalind::dna::DNA;
use rusty_rosalind::dna_string::DNAString;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap()]
    dna_string: String,
}

fn main() {
    let args = Args::parse();
    let dna_string: DNAString = args.dna_string.parse().unwrap();
    let result = dna_string.count_nucleotides();
    println!(
        "{:?} {:?} {:?} {:?}",
        result.get(&DNA::A).unwrap(),
        result.get(&DNA::C).unwrap(),
        result.get(&DNA::G).unwrap(),
        result.get(&DNA::T).unwrap()
    )
}
