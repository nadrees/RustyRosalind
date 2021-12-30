use std::convert::{From, TryFrom};
use std::fmt::Debug;
use std::hash::Hash;

/// A base Nucleotide
pub trait Nucleotide: TryFrom<char> + Sized + Eq + Hash + Clone + Debug {}

/// Indicates this nucleotide can be transcribed from another
pub trait Transcribable<'a, T>: Nucleotide + From<&'a T>
where
  T: 'a,
{
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum DNA {
  A,
  C,
  G,
  T,
}

impl Nucleotide for DNA {}

impl TryFrom<char> for DNA {
  type Error = char;

  fn try_from(c: char) -> Result<DNA, Self::Error> {
    match c {
      'A' => Ok(DNA::A),
      'C' => Ok(DNA::C),
      'G' => Ok(DNA::G),
      'T' => Ok(DNA::T),
      _ => Err(c),
    }
  }
}

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
