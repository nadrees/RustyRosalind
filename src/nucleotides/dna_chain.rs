use crate::nucleotides::DNA;
use std::fmt;

// A chain of DNA nucleotides
#[derive(Debug, PartialEq, Eq)]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::dna_chain::DNAChain;
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// let parsed_chain = DNAChain::parse_str(&"ACTG");
    /// let expected_chain = DNAChain::new(vec![DNA::A, DNA::C, DNA::T, DNA::G]);
    /// assert_eq!(parsed_chain, expected_chain);
    /// ```
    ///
    /// # Panics
    ///
    /// ```rust,should_panic
    /// use rusty_rosalind::nucleotides::dna_chain::DNAChain;
    ///
    /// DNAChain::parse_str(&"X");
    /// ```
    pub fn parse_str(s: &str) -> DNAChain {
        DNAChain::new(s.trim().chars().map(|c| DNA::parse(c)).collect())
    }

    /// computes the reverse compliment of the chain
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::dna_chain::DNAChain;
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// let expected_chain = DNAChain::new(vec![DNA::A, DNA::C, DNA::T, DNA::G]);
    /// let rev_compliment = DNAChain::new(vec![DNA::C, DNA::A, DNA::G, DNA::T]).reverse_compliment();
    /// assert_eq!(expected_chain, rev_compliment);
    /// ```
    pub fn reverse_compliment(&self) -> DNAChain {
        DNAChain::new(
            self.nucleotides
                .iter()
                .rev()
                .map(|n| n.compliment())
                .collect(),
        )
    }

    /// moves all of the nucleotides from other in to self, leaving
    /// other empty
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::dna_chain::DNAChain;
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// let mut chain1 = DNAChain::new(vec![DNA::A, DNA::C, DNA::T, DNA::G]);
    /// let chain2 = DNAChain::new(vec![DNA::A, DNA::C, DNA::T, DNA::G]);
    ///
    /// let expected_chain = DNAChain::new(vec![DNA::A, DNA::C, DNA::T, DNA::G, DNA::A, DNA::C, DNA::T, DNA::G]);
    /// assert_eq!(expected_chain, *chain1.append(chain2));
    /// ```
    pub fn append(&mut self, other: DNAChain) -> &DNAChain {
        self.nucleotides.append(&mut other.nucleotides.clone());
        self
    }

    /// computes the gc content of the current dna string
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::dna_chain::DNAChain;
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// let chain = DNAChain::new(vec![DNA::A, DNA::C, DNA::T, DNA::G]);
    /// let expected_result = 0.50f32;
    /// assert_eq!(expected_result, chain.gc_content());
    /// ```
    pub fn gc_content(&self) -> f32 {
        let gc_count = self
            .nucleotides
            .iter()
            .filter(|d| **d == DNA::G || **d == DNA::C)
            .count() as f32;
        let total_count = self.nucleotides.len() as f32;
        gc_count / total_count
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection::vec;
    use proptest::prelude::*;

    prop_compose! {
        fn arb_dna_chain(size: usize)(dna_chain in vec(any::<DNA>(), ..size)) -> DNAChain {
            DNAChain::new(dna_chain)
        }
    }

    proptest! {
        #[test]
        fn test_parse_str(dna_chain in arb_dna_chain(100)) {
            let dna_str = format!("{}", dna_chain);
            assert_eq!(dna_chain, DNAChain::parse_str(&dna_str));
        }

        #[test]
        fn test_append(dna_chain1 in arb_dna_chain(100), dna_chain2 in arb_dna_chain(100)) {
            let mut chain = dna_chain1.nucleotides.clone().to_owned();
            chain.append(&mut dna_chain2.nucleotides.clone().to_owned());
            let expected_chain = DNAChain::new(chain);

            let mut chain1 = DNAChain::new(dna_chain1.nucleotides.clone().to_owned());
            let chain2 = DNAChain::new(dna_chain2.nucleotides.clone().to_owned());

            assert_eq!(expected_chain, *chain1.append(chain2));
        }
    }
}
