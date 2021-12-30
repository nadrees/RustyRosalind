use rusty_rosalind::nucleotides::{DNA, RNA};
use rusty_rosalind::strands::Strand;
use std::collections::HashMap;

#[test]
fn test_dna_question() -> Result<(), char> {
  let parsed = "ATGCTTCAGAAAGGTCTTACG".parse::<Strand<DNA>>()?;
  let results = parsed.count_nucleotides();
  assert_eq!(
    results,
    HashMap::from([(&DNA::A, 6), (&DNA::C, 4), (&DNA::G, 5), (&DNA::T, 6)])
  );
  Ok(())
}

#[test]
fn test_rna_question() -> Result<(), char> {
  let dna_strand = "GATGGAACTTGACTACGTAAATT".parse::<Strand<DNA>>()?;
  let expected = "GAUGGAACUUGACUACGUAAAUU".parse::<Strand<RNA>>()?;
  let rna_strand: Strand<RNA> = dna_strand.transcribe();
  assert_eq!(rna_strand, expected);
  Ok(())
}
