use super::Nucleotide;
use crate::nucleotides::rna::RNA;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl AminoAcid {
    pub fn get_monoisotopic_mass(amino_acid: &AminoAcid) -> f64 {
        match amino_acid {
            AminoAcid::A => 71.03711,
            AminoAcid::C => 103.00919,
            AminoAcid::D => 115.02694,
            AminoAcid::E => 129.04259,
            AminoAcid::F => 147.06841,
            AminoAcid::G => 57.02146,
            AminoAcid::H => 137.05891,
            AminoAcid::I => 113.08406,
            AminoAcid::K => 128.09496,
            AminoAcid::L => 113.08406,
            AminoAcid::M => 131.04049,
            AminoAcid::N => 114.04293,
            AminoAcid::P => 97.05276,
            AminoAcid::Q => 128.05858,
            AminoAcid::R => 156.10111,
            AminoAcid::S => 87.03203,
            AminoAcid::T => 101.04768,
            AminoAcid::V => 99.06841,
            AminoAcid::W => 186.07931,
            AminoAcid::Y => 163.06333,
            AminoAcid::Stop => 0.0,
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
            (RNA::U, RNA::U, RNA::A) | (RNA::U, RNA::U, RNA::G) | (RNA::C, RNA::U, _) => {
                AminoAcid::L
            }
            (RNA::A, RNA::U, RNA::G) => AminoAcid::M,
            (RNA::A, RNA::A, RNA::U) | (RNA::A, RNA::A, RNA::C) => AminoAcid::N,
            (RNA::C, RNA::C, _) => AminoAcid::P,
            (RNA::C, RNA::A, RNA::A) | (RNA::C, RNA::A, RNA::G) => AminoAcid::Q,
            (RNA::C, RNA::G, _) | (RNA::A, RNA::G, RNA::A) | (RNA::A, RNA::G, RNA::G) => {
                AminoAcid::R
            }
            (RNA::U, RNA::C, _) | (RNA::A, RNA::G, RNA::U) | (RNA::A, RNA::G, RNA::C) => {
                AminoAcid::S
            }
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
