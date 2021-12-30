use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum DNA {
  A,
  C,
  G,
  T,
}

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
