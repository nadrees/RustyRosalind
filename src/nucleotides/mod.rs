/// The 4 base nucleotides that make up a DNA strand
pub enum DNA {
    A,
    C,
    G,
    T,
}

impl DNA {
    /// converts a character to its DNA strand, or panics
    pub fn parse(c: char) -> DNA {
        match c {
            'A' => DNA::A,
            'C' => DNA::C,
            'G' => DNA::G,
            'T' => DNA::T,
            x => panic!("Invalid DNA nucleotide: {:?}", x),
        }
    }
}

/// The 4 base nucleotides that make up an RNA strand
#[derive(Debug)]
pub enum RNA {
    A,
    C,
    G,
    U,
}

impl From<&DNA> for RNA {
    fn from(n: &DNA) -> RNA {
        match n {
            DNA::A => RNA::A,
            DNA::C => RNA::C,
            DNA::G => RNA::G,
            DNA::T => RNA::U,
        }
    }
}

pub mod dna_chain;
pub mod rna_chain;
