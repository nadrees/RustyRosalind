use crate::nucleotides::DNA;

// A chain of DNA nucleotides
pub struct DNAChain {
    nucleotides: Vec<DNA>,
}

impl IntoIterator for DNAChain {
    type Item = DNA;
    type IntoIter = ::std::vec::IntoIter<DNA>;

    fn into_iter(self) -> Self::IntoIter {
        self.nucleotides.into_iter()
    }
}

impl DNAChain {
    /// creates a new DNAChain with the specified nucleotides
    pub fn new(dna: Vec<DNA>) -> DNAChain {
        DNAChain { nucleotides: dna }
    }

    /// converts a string to a DNA string, or panics
    /// if any character is invalid.
    pub fn parse_str(s: &str) -> DNAChain {
        DNAChain::new(s.trim().chars().map(|c| DNA::parse(c)).collect())
    }
}
