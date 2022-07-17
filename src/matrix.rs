use std::{collections::HashMap, fmt};

use crate::{
    fasta::{read_records_from_result_lines, Fasta},
    nucleotides::Nucleotide,
    strands::Strand,
};

/// A collection of Fasta records. Provides operations
/// for comparing and summarizing the records
pub struct Matrix<T: Nucleotide> {
    pub records: Vec<Fasta<T>>,
}

impl<T: Nucleotide> Matrix<T> {
    /// creates a new Matrix from the provided Fasta records.
    /// panics if all of the records are not of the same length
    pub fn new(records: Vec<Fasta<T>>) -> Self {
        if !records.is_empty() {
            let strand_len = records[0].len();
            if records.iter().any(|r| r.len() != strand_len) {
                panic!("Expected all Fasta records to have the same length")
            }
        }
        Matrix { records }
    }

    /// reads the lines into a Matrix
    /// panics if any line is not Ok
    pub fn new_from_file_lines<TLines, TError>(lines: TLines) -> Self
    where
        TLines: Iterator<Item = Result<String, TError>>,
        TError: std::fmt::Debug,
        <T as TryFrom<char>>::Error: std::fmt::Debug,
    {
        Self::new(read_records_from_result_lines(lines))
    }

    /// Constructs the profile of the matrix
    pub fn profile(&self) -> Profile<T> {
        let mut data: HashMap<T, Vec<usize>> = HashMap::new();
        for record in &self.records {
            for i in 0..record.len() {
                let nucleotide = record.strand[i];
                if let Some(arr) = data.get_mut(&nucleotide) {
                    arr[i] += 1
                } else {
                    let mut arr = vec![0; record.len()];
                    arr[i] = 1;
                    data.insert(nucleotide, arr);
                }
            }
        }
        Profile::new(data)
    }

    /// Return the consensus of this matrix. See Profile.consensus.
    pub fn consensus(&self) -> Strand<T> {
        self.profile().consensus()
    }
}

/// The profile matix P is a N x L matrix where
/// N = the number of unique nucleotide values
/// L = the length of the strands
/// P(i,j) = the number of times the ith nucleotide appears in the jth position in a strand
///
/// Construct this by calling Matrix.profile()
#[derive(Debug, Eq, PartialEq)]
pub struct Profile<T: Nucleotide> {
    data: HashMap<T, Vec<usize>>,
    strand_len: usize,
}

impl<T: Nucleotide> Profile<T> {
    fn new(data: HashMap<T, Vec<usize>>) -> Self {
        let strand_len = if let Some(nucleotide) = data.keys().next() {
            data[nucleotide].len()
        } else {
            0
        };
        Profile { data, strand_len }
    }

    /// The consensus strand is the strand formed by taking the
    /// most common nucleotide at each position in the profile
    pub fn consensus(&self) -> Strand<T> {
        let mut strand: Vec<T> = Vec::with_capacity(self.strand_len);
        for i in 0..self.strand_len {
            let result = self.data.keys().fold(None, |accumulator, n| {
                let (nucleotide, lengths) = self.data.get_key_value(n).unwrap();
                match accumulator {
                    None => Some((*nucleotide, lengths[i])),
                    Some((_, l)) if l < lengths[i] => Some((*nucleotide, lengths[i])),
                    _ => accumulator,
                }
            });
            strand.insert(i, result.unwrap().0);
        }
        Strand::new(strand)
    }
}

impl<T: Nucleotide> fmt::Display for Profile<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<&T> = self.data.keys().collect();
        keys.sort();
        for key in keys {
            let values: Vec<String> = self.data[key].iter().map(|v| format!("{}", *v)).collect();
            writeln!(f, "{:?}: {}", key, values.join(" "))?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::nucleotides::dna::DNA;

    use super::*;

    #[test]
    fn test_profile() {
        let expected = Profile::new(HashMap::from([
            (DNA::A, vec![5, 1, 0, 0, 5, 5, 0, 0]),
            (DNA::C, vec![0, 0, 1, 4, 2, 0, 6, 1]),
            (DNA::G, vec![1, 1, 6, 3, 0, 1, 0, 0]),
            (DNA::T, vec![1, 5, 0, 0, 0, 1, 1, 6]),
        ]));
        let actual = Matrix::new(get_test_records()).profile();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_consensus() {
        let expected: Strand<DNA> = "ATGCAACT".parse().unwrap();
        let actual = Matrix::new(get_test_records()).consensus();
        assert_eq!(expected, actual);
    }

    fn get_test_records() -> Vec<Fasta<DNA>> {
        vec![
            Fasta {
                name: "Rosalind_1".to_string(),
                strand: "ATCCAGCT".parse().unwrap(),
            },
            Fasta {
                name: "Rosalind_2".to_string(),
                strand: "GGGCAACT".parse().unwrap(),
            },
            Fasta {
                name: "Rosalind_3".to_string(),
                strand: "ATGGATCT".parse().unwrap(),
            },
            Fasta {
                name: "Rosalind_4".to_string(),
                strand: "AAGCAACC".parse().unwrap(),
            },
            Fasta {
                name: "Rosalind_5".to_string(),
                strand: "TTGGAACT".parse().unwrap(),
            },
            Fasta {
                name: "Rosalind_6".to_string(),
                strand: "ATGCCATT".parse().unwrap(),
            },
            Fasta {
                name: "Rosalind_7".to_string(),
                strand: "ATGGCACT".parse().unwrap(),
            },
        ]
    }
}
