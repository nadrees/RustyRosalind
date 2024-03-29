use crate::nucleotides::{Complementable, Nucleotide, Transcribable};
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::str::FromStr;
use std::vec::Vec;

pub mod dna_strand;
pub mod protein_strand;
pub mod rna_strand;

#[derive(Eq, PartialEq)]
pub struct Strand<T: Nucleotide> {
    nucleotides: Vec<T>,
}

impl<T: Nucleotide> IntoIterator for Strand<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.nucleotides.into_iter()
    }
}

impl<T: Nucleotide> FromStr for Strand<T> {
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Strand<T>, Self::Err> {
        let parse_results = s.chars().map(|c| T::try_from(c)).collect();
        match parse_results {
            Ok(results) => Ok(Strand {
                nucleotides: results,
            }),
            Err(e) => Err(e),
        }
    }
}

impl<T: Nucleotide> Strand<T> {
    pub fn new(nucleotides: Vec<T>) -> Self {
        Strand {
            nucleotides: nucleotides,
        }
    }

    /// returns a new strand containing the two strands concatenated together
    pub fn concat(&self, other: &Strand<T>) -> Self {
        let mut nucleotides = self.nucleotides.clone();
        let other_copy = other.nucleotides.clone();
        nucleotides.extend(other_copy);
        Self::new(nucleotides)
    }

    pub fn push(&mut self, nucleotide: T) {
        self.nucleotides.push(nucleotide);
    }

    pub fn count_nucleotides(&self) -> HashMap<&T, u32> {
        let mut map: HashMap<&T, u32> = HashMap::new();
        for nucleotide in &self.nucleotides {
            let mut current_count = 0;
            if map.contains_key(nucleotide) {
                current_count = map.get(nucleotide).unwrap().to_owned();
            }
            map.insert(nucleotide, current_count + 1);
        }
        map
    }

    pub fn transcribe<'a, Rhs>(&'a self) -> Strand<Rhs>
    where
        Rhs: Transcribable<'a, T>,
        T: 'a,
    {
        Strand::new(self.nucleotides.iter().map(|n| Rhs::from(&n)).collect())
    }

    /// returns the Hamming distance between the 2 strings.
    /// panics if the strings are not of the same length
    pub fn distance(&self, other: &Self) -> usize {
        if self.nucleotides.len() != other.nucleotides.len() {
            panic!()
        }
        self.nucleotides
            .iter()
            .zip(other.nucleotides.iter())
            .filter(|(n1, n2)| **n1 != **n2)
            .count()
    }

    /// returns all indexes where this is a substring of other
    pub fn substrings(&self, other: &Strand<T>) -> Vec<usize> {
        let mut indexes = vec![];
        if other.nucleotides.len() < self.nucleotides.len() {
            return vec![];
        }
        for i in 0..(other.nucleotides.len() - self.nucleotides.len()) {
            if self.is_substr_starting_from(other, i) {
                indexes.push(i);
            }
        }
        indexes
    }

    pub fn is_substr_starting_from(&self, other: &Strand<T>, start_index: usize) -> bool {
        if other.nucleotides.len() <= self.nucleotides.len() + start_index {
            return false;
        }
        for i in 0..self.nucleotides.len() {
            if self.nucleotides[i] != other.nucleotides[i + start_index] {
                return false;
            }
        }
        true
    }

    /// returns true is the suffix of the current strand overlaps with the prefix of other
    ///
    /// @length: the length of the overlap to test for
    pub fn overlaps_with_length(&self, other: &Strand<T>, length: usize) -> bool {
        if length == 0 || self.nucleotides.len() < length || other.nucleotides.len() < length {
            return false;
        }
        for i in 0..length {
            if self.nucleotides[self.nucleotides.len() - length + i] != other.nucleotides[i] {
                return false;
            }
        }
        true
    }

    /// Returns the number of nucleotides in the strand, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.nucleotides.len()
    }
}

impl<T: Complementable> Strand<T> {
    pub fn reverse_compliment(&self) -> Strand<T> {
        let mut rev = self.nucleotides.clone();
        rev.reverse();
        Strand::new(rev.into_iter().map(|n| T::compliment(&n)).collect())
    }
}

impl<T: Nucleotide> fmt::Debug for Strand<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.nucleotides).finish()
    }
}

impl<T: Nucleotide> fmt::Display for Strand<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.nucleotides
            .iter()
            .map(|n| write!(f, "{:?}", n))
            .collect()
    }
}

impl<T: Nucleotide> Index<usize> for Strand<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.nucleotides[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nucleotides::dna::DNA;

    #[test]
    fn test_count_nucleotides() {
        let dna_string = Strand {
            nucleotides: vec![
                DNA::A,
                DNA::C,
                DNA::C,
                DNA::G,
                DNA::G,
                DNA::G,
                DNA::T,
                DNA::T,
                DNA::T,
                DNA::T,
            ],
        };

        let results = dna_string.count_nucleotides();

        assert_eq!(results.get(&DNA::A), Some(&1));
        assert_eq!(results.get(&DNA::C), Some(&2));
        assert_eq!(results.get(&DNA::G), Some(&3));
        assert_eq!(results.get(&DNA::T), Some(&4));
    }

    #[test]
    fn test_reverse_compliment() -> Result<(), char> {
        let dna_string: Strand<DNA> = "AAAACCCGGT".parse()?;
        let expected: Strand<DNA> = "ACCGGGTTTT".parse()?;
        assert_eq!(dna_string.reverse_compliment(), expected);
        Ok(())
    }

    macro_rules! is_substr_starting_at_test {
    ($($name:ident: $value:expr,)*) => {
      $(
        #[test]
        fn $name() -> Result<(), char> {
          let (str_1, str_2, start_index, expected) = $value;
          let strand_1: Strand<DNA> = str_1.parse()?;
          let strand_2: Strand<DNA> = str_2.parse()?;
          assert_eq!(strand_1.is_substr_starting_from(&strand_2, start_index), expected);
          Ok(())
        }
      )*
    };
  }

    is_substr_starting_at_test! {
      is_substr_starting_at_str_1_longer: ("GATATATGCATATACTT", "ATAT", 0, false),
      is_substr_starting_at_wrong_index: ("ATAT", "GATATATGCATATACTT", 0, false),
      is_substr_starting_at_right_index: ("ATAT", "GATATATGCATATACTT", 1, true),
    }

    #[test]
    fn test_substrs() -> Result<(), char> {
        let strand_1: Strand<DNA> = "GATATATGCATATACTT".parse()?;
        let strand_2: Strand<DNA> = "ATAT".parse()?;
        assert_eq!(strand_2.substrings(&strand_1), vec![1, 3, 9]);
        Ok(())
    }

    #[test]
    fn test_distance() -> Result<(), char> {
        let strand_1: Strand<DNA> = "GAGCCTACTAACGGGAT".parse()?;
        let strand_2: Strand<DNA> = "CATCGTAATGACGGCCT".parse()?;
        assert_eq!(strand_1.distance(&strand_2), 7);
        Ok(())
    }

    mod overlaps_with_length_tests {
        use super::*;

        macro_rules! overlaps_with_length_test {
      ($($name:ident: $value:expr,)*) => {
        $(
          #[test]
          fn $name() -> Result<(), char> {
            let (str_1, str_2, overlap_length, expected) = $value;
            let strand_1: Strand<DNA> = str_1.parse()?;
            let strand_2: Strand<DNA> = str_2.parse()?;
            assert_eq!(strand_1.overlaps_with_length(&strand_2, overlap_length), expected);
            Ok(())
          }
        )*
      };
    }

        overlaps_with_length_test! {
          overlaps: ("AAATAAA", "AAATTTT", 3, true),
          no_overlap: ("AAATAAA", "AAATTTT", 4, false),
          length_zero: ("AAATAAA", "AAATTTT", 0, false),
          length_too_long: ("AAATAAA", "AAATTTT", 10, false),
        }
    }
}
