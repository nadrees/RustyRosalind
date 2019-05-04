use crate::nucleotides::dna_chain::DNAChain;
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
/// Represents a parsing error when converting a string
/// to Record
pub struct ParseRecordError {
    message: String,
}

impl fmt::Display for ParseRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ParseRecordError {}

impl ParseRecordError {
    fn new(message: &str) -> ParseRecordError {
        ParseRecordError {
            message: message.to_owned(),
        }
    }
}

/// A DNA record in the standard FASTA format
#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub dna_string: DNAChain,
}

impl FromStr for Record {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if let Some(first_line) = lines.next() {
            let record_name = Record::parse_name(first_line)?;
            let mut dna_chain = DNAChain::new(vec![]);
            for dna_line in lines.map(|line| DNAChain::parse_str(line)) {
                dna_chain.append(dna_line);
            }
            Ok(Record {
                name: record_name.to_owned(),
                dna_string: dna_chain,
            })
        } else {
            Err(ParseRecordError::new("Expected a non-empty string"))
        }
    }
}

impl Record {
    /// parses a string containing one or more records in the FASTA format
    /// in to a vector of Records
    pub fn parse_records(s: &str) -> Result<Vec<Record>, ParseRecordError> {
        let mut start_indexes = s.match_indices(">").map(|(index, _)| index).peekable();
        let mut records: Vec<Record> = vec![];

        while let Some(start_index) = start_indexes.next() {
            let record = match start_indexes.peek() {
                Some(end_index) => Record::from_str(&s[start_index..*end_index]),
                None => Record::from_str(&s[start_index..]),
            }?;
            records.push(record);
        }

        Ok(records)
    }

    fn parse_name(s: &str) -> Result<&str, ParseRecordError> {
        let mut chars = s.chars();
        let first_char = chars.next();
        match first_char {
            None => Err(ParseRecordError::new(
                "First line of record must start with a '>', received None",
            )),
            Some(first_char) => {
                if first_char != '>' {
                    Err(ParseRecordError::new(&format!(
                        "First line of a record must start with a '>', recieved {}",
                        first_char,
                    )))
                } else {
                    Ok(chars.as_str())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nucleotides::DNA;
    use proptest::collection::vec;
    use proptest::prelude::*;

    prop_compose! {
        fn arb_dna_lines(num_lines: usize)(lines in vec(any::<Vec<DNA>>(), num_lines)) -> Vec<Vec<DNA>> {
            lines
        }
    }

    prop_compose! {
        fn arb_record_str()(dna_lines in arb_dna_lines(10), name in "[^>\n\r]*") -> (String, DNAChain, String) {
            let first_line = format!(">{}", name);
            let rest_lines = dna_lines.iter()
                .map(|l| l.iter().map(|d| format!("{:?}", d)).collect::<Vec<String>>().join(""))
                .collect::<Vec<String>>()
                .join("\n");
            let lines = format!("{}\n{}", first_line, rest_lines);

            let flattened_lines = dna_lines.clone().to_owned().into_iter().flatten().collect();

            (name, DNAChain::new(flattened_lines), lines)
        }
    }

    prop_compose! {
        fn arb_records(max_num: usize)(records in vec(arb_record_str(), 1..max_num)) -> (String, Vec<Record>) {
            let full_str = records.iter()
                .map(|(_1, _2, record_str)| record_str.to_owned())
                .collect::<Vec<String>>()
                .join("\n");
            let records = records.into_iter()
                .map(|(name, nucleotides, _)| Record {
                    name: name,
                    dna_string: nucleotides
                })
                .collect();
            (full_str, records)
        }
    }

    proptest! {
        #[test]
        fn test_from_str((name, dna_lines, record_str) in arb_record_str()) {
            let parsed_record = Record::from_str(&record_str)?;
            assert_eq!(name, parsed_record.name);
            assert_eq!(dna_lines, parsed_record.dna_string);
        }

        #[test]
        fn test_parse_str((records_str, records) in arb_records(10)) {
            let parsed_records = Record::parse_records(&records_str)?;
            assert_eq!(records, parsed_records);
        }
    }
}
