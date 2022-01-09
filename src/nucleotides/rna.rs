use super::dna::DNA;
use super::protein::AminoAcid;
use super::{Nucleotide, Transcribable};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum RNA {
  A,
  C,
  G,
  U,
}

impl RNA {
  /// returns all possible reverse translations from an amino acid
  /// to the 3-tuple of RNA nucleotides
  pub fn reverse_translations(protein: &AminoAcid) -> Vec<(Self, Self, Self)> {
    match protein {
      AminoAcid::A => vec![
        (RNA::G, RNA::C, RNA::A),
        (RNA::G, RNA::C, RNA::C),
        (RNA::G, RNA::C, RNA::G),
        (RNA::G, RNA::C, RNA::U),
      ],
      AminoAcid::C => vec![(RNA::U, RNA::G, RNA::U), (RNA::U, RNA::G, RNA::C)],
      AminoAcid::D => vec![(RNA::G, RNA::A, RNA::U), (RNA::G, RNA::A, RNA::C)],
      AminoAcid::E => vec![(RNA::G, RNA::A, RNA::A), (RNA::G, RNA::A, RNA::G)],
      AminoAcid::F => vec![(RNA::U, RNA::U, RNA::U), (RNA::U, RNA::U, RNA::C)],
      AminoAcid::G => vec![
        (RNA::G, RNA::G, RNA::A),
        (RNA::G, RNA::G, RNA::C),
        (RNA::G, RNA::G, RNA::G),
        (RNA::G, RNA::G, RNA::U),
      ],
      AminoAcid::H => vec![(RNA::C, RNA::A, RNA::U), (RNA::C, RNA::A, RNA::C)],
      AminoAcid::I => vec![
        (RNA::A, RNA::U, RNA::U),
        (RNA::A, RNA::U, RNA::C),
        (RNA::A, RNA::U, RNA::A),
      ],
      AminoAcid::K => vec![(RNA::A, RNA::A, RNA::A), (RNA::A, RNA::A, RNA::G)],
      AminoAcid::L => vec![
        (RNA::U, RNA::U, RNA::A),
        (RNA::U, RNA::U, RNA::G),
        (RNA::C, RNA::U, RNA::A),
        (RNA::C, RNA::U, RNA::C),
        (RNA::C, RNA::U, RNA::G),
        (RNA::C, RNA::U, RNA::U),
      ],
      AminoAcid::M => vec![(RNA::A, RNA::U, RNA::G)],
      AminoAcid::N => vec![(RNA::A, RNA::A, RNA::U), (RNA::A, RNA::A, RNA::C)],
      AminoAcid::P => vec![
        (RNA::C, RNA::C, RNA::A),
        (RNA::C, RNA::C, RNA::C),
        (RNA::C, RNA::C, RNA::G),
        (RNA::C, RNA::C, RNA::U),
      ],
      AminoAcid::Q => vec![(RNA::C, RNA::A, RNA::A), (RNA::C, RNA::A, RNA::G)],
      AminoAcid::R => {
        vec![
          (RNA::C, RNA::G, RNA::A),
          (RNA::C, RNA::G, RNA::C),
          (RNA::C, RNA::G, RNA::G),
          (RNA::C, RNA::G, RNA::U),
          (RNA::A, RNA::G, RNA::A),
          (RNA::A, RNA::G, RNA::G),
        ]
      }
      AminoAcid::S => {
        vec![
          (RNA::U, RNA::C, RNA::A),
          (RNA::U, RNA::C, RNA::C),
          (RNA::U, RNA::C, RNA::G),
          (RNA::U, RNA::C, RNA::U),
          (RNA::A, RNA::G, RNA::U),
          (RNA::A, RNA::G, RNA::C),
        ]
      }
      AminoAcid::T => vec![
        (RNA::A, RNA::C, RNA::A),
        (RNA::A, RNA::C, RNA::C),
        (RNA::A, RNA::C, RNA::G),
        (RNA::A, RNA::C, RNA::U),
      ],
      AminoAcid::V => vec![
        (RNA::G, RNA::U, RNA::A),
        (RNA::G, RNA::U, RNA::C),
        (RNA::G, RNA::U, RNA::G),
        (RNA::G, RNA::U, RNA::U),
      ],
      AminoAcid::W => vec![(RNA::U, RNA::G, RNA::G)],
      AminoAcid::Y => vec![(RNA::U, RNA::A, RNA::U), (RNA::U, RNA::A, RNA::C)],
      AminoAcid::Stop => {
        vec![
          (RNA::U, RNA::A, RNA::A),
          (RNA::U, RNA::A, RNA::G),
          (RNA::U, RNA::G, RNA::A),
        ]
      }
    }
  }
}

impl Nucleotide for RNA {}

impl<'a> Transcribable<'a, DNA> for RNA {}

impl TryFrom<char> for RNA {
  type Error = char;

  fn try_from(c: char) -> Result<RNA, Self::Error> {
    match c {
      'A' => Ok(RNA::A),
      'C' => Ok(RNA::C),
      'G' => Ok(RNA::G),
      'U' => Ok(RNA::U),
      _ => Err(c),
    }
  }
}

impl From<&DNA> for RNA {
  fn from(dna: &DNA) -> Self {
    match dna {
      &DNA::A => RNA::A,
      &DNA::C => RNA::C,
      &DNA::G => RNA::G,
      &DNA::T => RNA::U,
    }
  }
}
