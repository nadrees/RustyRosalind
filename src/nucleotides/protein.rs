use super::Nucleotide;
use crate::nucleotides::rna::RNA;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum AminoAcid {
  A,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  K,
  L,
  M,
  N,
  P,
  Q,
  R,
  S,
  T,
  V,
  W,
  Y,
  Stop,
}

impl Nucleotide for AminoAcid {}

impl TryFrom<char> for AminoAcid {
  type Error = char;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      'A' => Ok(AminoAcid::A),
      'C' => Ok(AminoAcid::C),
      'D' => Ok(AminoAcid::D),
      'E' => Ok(AminoAcid::E),
      'F' => Ok(AminoAcid::F),
      'G' => Ok(AminoAcid::G),
      'H' => Ok(AminoAcid::H),
      'I' => Ok(AminoAcid::I),
      'K' => Ok(AminoAcid::K),
      'L' => Ok(AminoAcid::L),
      'M' => Ok(AminoAcid::M),
      'N' => Ok(AminoAcid::N),
      'P' => Ok(AminoAcid::P),
      'Q' => Ok(AminoAcid::Q),
      'R' => Ok(AminoAcid::R),
      'S' => Ok(AminoAcid::S),
      'T' => Ok(AminoAcid::T),
      'V' => Ok(AminoAcid::V),
      'W' => Ok(AminoAcid::W),
      'Y' => Ok(AminoAcid::Y),
      _ => Err(c),
    }
  }
}

impl From<(RNA, RNA, RNA)> for AminoAcid {
  fn from(codon: (RNA, RNA, RNA)) -> Self {
    match codon {
      (RNA::G, RNA::C, _) => AminoAcid::A,
      (RNA::U, RNA::G, RNA::U) | (RNA::U, RNA::G, RNA::C) => AminoAcid::C,
      (RNA::G, RNA::A, RNA::U) | (RNA::G, RNA::A, RNA::C) => AminoAcid::D,
      (RNA::G, RNA::A, RNA::A) | (RNA::G, RNA::A, RNA::G) => AminoAcid::E,
      (RNA::U, RNA::U, RNA::U) | (RNA::U, RNA::U, RNA::C) => AminoAcid::F,
      (RNA::G, RNA::G, _) => AminoAcid::G,
      (RNA::C, RNA::A, RNA::U) | (RNA::C, RNA::A, RNA::C) => AminoAcid::H,
      (RNA::A, RNA::U, RNA::U) | (RNA::A, RNA::U, RNA::C) | (RNA::A, RNA::U, RNA::A) => {
        AminoAcid::I
      }
      (RNA::A, RNA::A, RNA::A) | (RNA::A, RNA::A, RNA::G) => AminoAcid::K,
      (RNA::U, RNA::U, RNA::A) | (RNA::U, RNA::U, RNA::G) | (RNA::C, RNA::U, _) => AminoAcid::L,
      (RNA::A, RNA::U, RNA::G) => AminoAcid::M,
      (RNA::A, RNA::A, RNA::U) | (RNA::A, RNA::A, RNA::C) => AminoAcid::N,
      (RNA::C, RNA::C, _) => AminoAcid::P,
      (RNA::C, RNA::A, RNA::A) | (RNA::C, RNA::A, RNA::G) => AminoAcid::Q,
      (RNA::C, RNA::G, _) | (RNA::A, RNA::G, RNA::A) | (RNA::A, RNA::G, RNA::G) => AminoAcid::R,
      (RNA::U, RNA::C, _) | (RNA::A, RNA::G, RNA::U) | (RNA::A, RNA::G, RNA::C) => AminoAcid::S,
      (RNA::A, RNA::C, _) => AminoAcid::T,
      (RNA::G, RNA::U, _) => AminoAcid::V,
      (RNA::U, RNA::G, RNA::G) => AminoAcid::W,
      (RNA::U, RNA::A, RNA::U) | (RNA::U, RNA::A, RNA::C) => AminoAcid::Y,
      (RNA::U, RNA::A, RNA::A) | (RNA::U, RNA::A, RNA::G) | (RNA::U, RNA::G, RNA::A) => {
        AminoAcid::Stop
      }
    }
  }
}
