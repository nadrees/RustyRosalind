use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub enum DNA {
  A,
  C,
  G,
  T,
}

#[derive(PartialEq, Eq, Hash)]
pub enum RNA {
  A,
  C,
  G,
  U,
}

impl From<&DNA> for RNA {
  fn from(n: &DNA) -> RNA {
    match n {
      DNA::A => RNA::A,
      DNA::C => RNA::C,
      DNA::G => RNA::G,
      DNA::T => RNA::U,
    }
  }
}

impl fmt::Display for RNA {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match &self {
        RNA::A => 'A',
        RNA::C => 'C',
        RNA::G => 'G',
        RNA::U => 'U',
      }
    )
  }
}

pub struct DNAChain(Vec<DNA>);

impl DNAChain {
  pub fn parse_nucleotides(input: &str) -> DNAChain {
    DNAChain(
      input
        .trim()
        .chars()
        .map(|c| match c {
          'A' => DNA::A,
          'C' => DNA::C,
          'G' => DNA::G,
          'T' => DNA::T,
          x => panic!("Invalid character read: {}", x),
        })
        .collect(),
    )
  }
}

impl IntoIterator for DNAChain {
  type Item = DNA;
  type IntoIter = ::std::vec::IntoIter<DNA>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

pub struct RNAChain(Vec<RNA>);

impl From<DNAChain> for RNAChain {
  fn from(other: DNAChain) -> RNAChain {
    RNAChain(other.0.iter().map(|c| RNA::from(c)).collect())
  }
}

impl fmt::Display for RNAChain {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let formatted_results: Vec<String> = self.0.iter().map(|c| format!("{}", c)).collect();
    write!(f, "{}", formatted_results.join(""))
  }
}
