use crate::nucleotides::dna::DNA;
use crate::strands::Strand;

impl Strand<DNA> {
    pub fn get_gc_content(&self) -> f32 {
        let gc_count = self
            .nucleotides
            .iter()
            .filter(|n| **n == DNA::G || **n == DNA::C)
            .count();
        gc_count as f32 / self.nucleotides.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_content() -> Result<(), char> {
        let dna_string: Strand<DNA> =
      "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT"
        .parse()?;
        assert_eq!(dna_string.get_gc_content() * 100.0, 60.919540);
        Ok(())
    }
}
