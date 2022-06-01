use crate::nucleotides::protein::AminoAcid;
use crate::nucleotides::rna::RNA;
use crate::strands::rna_strand::RNAReverseTranslations;
use crate::strands::Strand;

impl Strand<AminoAcid> {
  pub fn reverse_translations(&self) -> RNAReverseTranslations {
    RNAReverseTranslations::new(self)
  }

  pub fn weight(&self) -> f64 {
    self
      .nucleotides
      .iter()
      .map(|aa| AminoAcid::get_monoisotopic_mass(aa))
      .sum()
  }
}

impl From<Strand<RNA>> for Strand<AminoAcid> {
  fn from(rna_strand: Strand<RNA>) -> Self {
    if rna_strand.nucleotides.len() % 3 != 0 {
      panic!()
    }
    Strand {
      nucleotides: rna_strand
        .nucleotides
        .chunks_exact(3)
        .map(|chunk| AminoAcid::from((chunk[0], chunk[1], chunk[2])))
        .skip_while(|aa| *aa != AminoAcid::M)
        .take_while(|aa| *aa != AminoAcid::Stop)
        .collect(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_rna_strand() -> Result<(), char> {
    let rna_strand: Strand<RNA> = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA".parse()?;
    let actual: Strand<AminoAcid> = Strand::from(rna_strand);
    let expected: Strand<AminoAcid> = "MAMAPRTEINSTRING".parse()?;
    assert_eq!(actual, expected);
    Ok(())
  }
}
