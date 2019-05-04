/// The 4 base nucleotides that make up a DNA strand
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DNA {
    A,
    C,
    G,
    T,
}

impl DNA {
    /// converts a character to its DNA strand, or panics
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// assert_eq!(DNA::parse('A'), DNA::A);
    /// assert_eq!(DNA::parse('C'), DNA::C);
    /// assert_eq!(DNA::parse('G'), DNA::G);
    /// assert_eq!(DNA::parse('T'), DNA::T);
    /// ```
    ///
    /// # Panics
    ///
    /// ```rust,should_panic
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// DNA::parse('X');
    /// ```
    pub fn parse(c: char) -> DNA {
        match c {
            'A' => DNA::A,
            'C' => DNA::C,
            'G' => DNA::G,
            'T' => DNA::T,
            x => panic!("Invalid DNA nucleotide: {:?}", x),
        }
    }

    /// computes the compliment of the current DNA nucleotide:
    /// A <=> T, C <=> G
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::DNA;
    ///
    /// assert_eq!(DNA::A.compliment(), DNA::T);
    /// assert_eq!(DNA::T.compliment(), DNA::A);
    /// assert_eq!(DNA::G.compliment(), DNA::C);
    /// assert_eq!(DNA::C.compliment(), DNA::G);
    /// ```
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
#[derive(Debug, PartialEq, Eq)]
pub enum RNA {
    A,
    C,
    G,
    U,
}

impl From<&DNA> for RNA {
    /// Converts a DNA nucleotide to an RNA
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_rosalind::nucleotides::{DNA, RNA};
    ///
    /// assert_eq!(RNA::from(&DNA::A), RNA::A);
    /// assert_eq!(RNA::from(&DNA::C), RNA::C);
    /// assert_eq!(RNA::from(&DNA::G), RNA::G);
    /// assert_eq!(RNA::from(&DNA::T), RNA::U);
    /// ```
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use proptest::arbitrary::Arbitrary;
    use proptest::prelude::*;

    impl Arbitrary for DNA {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(DNA::A), Just(DNA::C), Just(DNA::G), Just(DNA::T)].boxed()
        }
    }
}
