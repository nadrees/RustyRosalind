use crate::dna::DNA;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;

pub struct DNAString {
  nucleotides: Vec<DNA>,
}

impl FromStr for DNAString {
  type Err = char;

  fn from_str(s: &str) -> Result<DNAString, Self::Err> {
    let parse_results = s.chars().map(|c| DNA::try_from(c)).collect();
    match parse_results {
      Ok(results) => Ok(DNAString {
        nucleotides: results,
      }),
      Err(e) => Err(e),
    }
  }
}

impl DNAString {
  pub fn count_nucleotides(&self) -> HashMap<DNA, u32> {
    let mut map = HashMap::from([(DNA::A, 0), (DNA::C, 0), (DNA::G, 0), (DNA::T, 0)]);
    for nucleotide in &self.nucleotides {
      let current_count = map.get(nucleotide).unwrap().to_owned();
      map.insert(*nucleotide, current_count + 1);
    }
    map
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_nucleotides() {
    let dna_string = DNAString {
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
}
