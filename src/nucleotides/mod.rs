/// The 4 base nucleotides that make up a DNA strand
#[derive(Debug)]
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

    /// computes the compliment of the current DNA nucleotide
    /// A <=> T, C <=> G
    pub fn compliment(&self) -> DNA {
        match &self {
            DNA::A => DNA::T,
            DNA::T => DNA::A,
            DNA::C => DNA::G,
            DNA::G => DNA::C,
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
