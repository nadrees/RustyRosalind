use super::dna::DNA;
use super::{Nucleotide, Transcribable};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum RNA {
  A,
  C,
  G,
  U,
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
