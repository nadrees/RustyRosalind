use super::nucleotides::Nucleotide;
use super::strands::Strand;

/// a record in the FASTA format
pub struct Fasta<T: Nucleotide> {
  pub name: String,
  pub strand: Strand<T>,
}

/// reads the lines into a list of FASTA records.
/// panics if any line is not Ok
pub fn read_records_from_result_lines<TNucleotide, TLines, TError>(
  lines: TLines,
) -> Vec<Fasta<TNucleotide>>
where
  TNucleotide: Nucleotide,
  TLines: Iterator<Item = Result<String, TError>>,
  TError: std::fmt::Debug,
  <TNucleotide as TryFrom<char>>::Error: std::fmt::Debug,
{
  let unwrapped_lines = lines
    .into_iter()
    .map(|l| l.unwrap())
    .collect::<Vec<String>>();
  read_records(unwrapped_lines.iter().map(|l| &l[..]))
}

/// reads the lines into a list of FASTA records
pub fn read_records<'a, TNucleotide, TLines>(lines: TLines) -> Vec<Fasta<TNucleotide>>
where
  TNucleotide: Nucleotide,
  TLines: Iterator<Item = &'a str>,
  <TNucleotide as TryFrom<char>>::Error: std::fmt::Debug,
{
  let mut records: Vec<Fasta<TNucleotide>> = vec![];
  let mut current_lines: Vec<&str> = vec![];
  for line in lines {
    if line.starts_with(">") {
      if !current_lines.is_empty() {
        records.push(Fasta::from(current_lines.into_iter()));
      }
      current_lines = vec![];
    }
    current_lines.push(line);
  }
  if !current_lines.is_empty() {
    records.push(Fasta::from(current_lines.into_iter()));
  }
  records
}

impl<'a, TNucleotide, TIterator> From<TIterator> for Fasta<TNucleotide>
where
  TNucleotide: Nucleotide,
  TIterator: IntoIterator<Item = &'a str>,
  <TNucleotide as TryFrom<char>>::Error: std::fmt::Debug,
{
  fn from(lines: TIterator) -> Self {
    let mut lines_iter = lines.into_iter();
    let first_line = lines_iter.next().unwrap();
    Fasta {
      name: parse_name(first_line).to_string(),
      strand: lines_iter.collect::<Vec<&str>>().join("").parse().unwrap(),
    }
  }
}

fn parse_name(line: &str) -> &str {
  if !line.starts_with(">") {
    panic!();
  }
  &line[1..]
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::nucleotides::dna::DNA;

  #[test]
  fn test_from() {
    let result: Fasta<DNA> = Fasta::from(vec![
      ">Rosalind_6404",
      "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC",
      "TCCCACTAATAATTCTGAGG",
    ]);
    assert_eq!(result.name, "Rosalind_6404");
    assert_eq!(
      format!("{}", result.strand),
      "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG"
    );
  }

  #[test]
  fn test_read_records() {
    let records: Vec<Fasta<DNA>> = read_records(
      vec![
        ">Rosalind_6404",
        "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC",
        "TCCCACTAATAATTCTGAGG",
        ">Rosalind_5959",
        "CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT",
        "ATATCCATTTGTCAGCAGACACGC",
        ">Rosalind_0808",
        "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC",
        "TGGGAACCTGCGGGCAGTAGGTGGAAT",
      ]
      .into_iter(),
    );
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].name, "Rosalind_6404");
  }
}
