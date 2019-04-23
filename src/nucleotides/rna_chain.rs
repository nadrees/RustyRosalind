use crate::nucleotides::dna_chain::DNAChain;
use crate::nucleotides::RNA;
use std::fmt;

// A chain of RNA nucleotides
pub struct RNAChain {
    nucleotides: Vec<RNA>,
}

impl RNAChain {
    /// creates a new RNA chain with the specified nucleotides
    pub fn new(rna: Vec<RNA>) -> RNAChain {
        RNAChain { nucleotides: rna }
    }
}

impl From<DNAChain> for RNAChain {
    fn from(dna_chain: DNAChain) -> RNAChain {
        RNAChain::new(dna_chain.into_iter().map(|d| RNA::from(&d)).collect())
    }
}

impl fmt::Display for RNAChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_nucleotides: Vec<String> = self
            .nucleotides
            .iter()
            .map(|n| format!("{:?}", n))
            .collect();
        write!(f, "{}", formatted_nucleotides.join(""))
    }
}
