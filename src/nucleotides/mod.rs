use std::convert::{From, TryFrom};
use std::fmt::Debug;
use std::hash::Hash;

pub mod dna;
pub mod protein;
pub mod rna;

/// A base Nucleotide. For convenience, Protein encodings are also considered
/// a nucleotide.
pub trait Nucleotide: TryFrom<char> + Sized + Eq + Hash + Clone + Debug {}

/// Indicates this nucleotide can be transcribed from another
pub trait Transcribable<'a, T>: Nucleotide + From<&'a T>
where
    T: 'a,
{
}

/// Nucleotides implementing this trait have compliments of each other.
pub trait Complementable: Nucleotide {
    /// Returns the compliment of the current nucleotide
    fn compliment(&self) -> Self;
}
