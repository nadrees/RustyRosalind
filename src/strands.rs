use crate::nucleotides::dna::DNA;
use crate::nucleotides::{Complementable, Nucleotide, Transcribable};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Eq, PartialEq)]
pub struct Strand<T: Nucleotide> {
  nucleotides: Vec<T>,
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
    Strand {
      nucleotides: self.nucleotides.iter().map(|n| Rhs::from(&n)).collect(),
    }
  }
}

impl Strand<DNA> {
  pub fn get_gc_content(&self) -> f32 {
    let gc_count = self
      .nucleotides
      .iter()
      .filter(|n| **n == DNA::G || **n == DNA::C)
      .count();
    gc_count as f32 / self.nucleotides.len() as f32
  }
}

impl<T: Complementable> Strand<T> {
  pub fn reverse_compliment(&self) -> Strand<T> {
    let mut rev = self.nucleotides.clone();
    rev.reverse();
    Strand {
      nucleotides: rev.into_iter().map(|n| T::compliment(&n)).collect(),
    }
  }
}

impl<T: Nucleotide> fmt::Debug for Strand<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(&self.nucleotides).finish()
  }
}

impl<T: Nucleotide> fmt::Display for Strand<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self
      .nucleotides
      .iter()
      .map(|n| write!(f, "{:?}", n))
      .collect()
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

  #[test]
  fn test_gc_content() -> Result<(), char> {
    let dna_string: Strand<DNA> =
      "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT"
        .parse()?;
    assert_eq!(dna_string.get_gc_content() * 100.0, 60.919540);
    Ok(())
  }
}
