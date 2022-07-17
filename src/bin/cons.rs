use clap::Parser;
use rusty_rosalind::{args::FileArgs as Args, matrix::Matrix, nucleotides::dna::DNA};

fn main() {
    let args = Args::parse();
    let lines = args.read_file().unwrap();
    let records: Matrix<DNA> = Matrix::new_from_file_lines(lines);
    println!("{}", records.consensus());
    println!("{}", records.profile());
}
