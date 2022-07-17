use super::{Complementable, Nucleotide};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl Complementable for DNA {
    fn compliment(&self) -> Self {
        match &self {
            &DNA::A => DNA::T,
            &DNA::T => DNA::A,
            &DNA::C => DNA::G,
            &DNA::G => DNA::C,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliment_a() {
        assert_eq!(DNA::A.compliment(), DNA::T);
        assert_eq!(DNA::A.compliment().compliment(), DNA::A);
    }

    #[test]
    fn test_compliment_c() {
        assert_eq!(DNA::C.compliment(), DNA::G);
        assert_eq!(DNA::C.compliment().compliment(), DNA::C);
    }
}
