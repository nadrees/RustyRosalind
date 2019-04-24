use crate::nucleotides::DNA;
use std::fmt;

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

    /// computes the reverse compliment of the chain
    pub fn reverse_compliment(&self) -> DNAChain {
        DNAChain::new(
            self.nucleotides
                .iter()
                .rev()
                .map(|n| n.compliment())
                .collect(),
        )
    }
}

impl fmt::Display for DNAChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_nucleotides: Vec<String> = self
            .nucleotides
            .iter()
            .map(|d| format!("{:?}", d))
            .collect();
        write!(f, "{}", formatted_nucleotides.join(""))
    }
}
