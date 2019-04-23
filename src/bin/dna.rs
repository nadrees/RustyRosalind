use clap::{App, Arg};
use rusty_rosalind::nucleotides::DNA;
use std::fs;

struct Counts {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
}

fn main() {
    let matches = App::new("DNA")
        .arg(Arg::with_name("input_file").required(true).index(1))
        .get_matches();
    let f_name = matches.value_of("input_file").unwrap();
    let dna = DNA::parse_str(fs::read_to_string(f_name).unwrap().as_ref());
    let result = dna.iter().fold(
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
