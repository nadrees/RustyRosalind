use rusty_rosalind::dna::DNA;
use rusty_rosalind::dna_string::DNAString;
use std::collections::HashMap;
use std::str::FromStr;

#[cfg(test)]
#[test]
fn test_dna_question() -> Result<(), char> {
  let results = DNAString::from_str(&r"ATGCTTCAGAAAGGTCTTACG")?.count_nucleotides();
  assert_eq!(
    results,
    HashMap::from([(DNA::A, 6), (DNA::C, 4), (DNA::G, 5), (DNA::T, 6)])
  );
  Ok(())
}
