use rusty_rosalind::nucleotides::{dna_chain::DNAChain, DNA};
use rusty_rosalind::start_standard_program;

struct Counts {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
}

fn main() {
    let dna = DNAChain::parse_str(&start_standard_program!("DNA"));
    let result = dna.into_iter().fold(
        Counts {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        },
        |current_counts, nucleotide| match nucleotide {
            DNA::A => Counts {
                a: current_counts.a + 1,
                ..current_counts
            },
            DNA::C => Counts {
                c: current_counts.c + 1,
                ..current_counts
            },
            DNA::G => Counts {
                g: current_counts.g + 1,
                ..current_counts
            },
            DNA::T => Counts {
                t: current_counts.t + 1,
                ..current_counts
            },
        },
    );
    println!("{} {} {} {}", result.a, result.c, result.g, result.t);
}
