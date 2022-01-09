use super::Strand;
use crate::nucleotides::protein::AminoAcid;
use crate::nucleotides::rna::RNA;

pub struct RNAReverseTranslations {
  indexes: Vec<usize>,
  possible_translations: Vec<Vec<(RNA, RNA, RNA)>>,
}

impl Iterator for RNAReverseTranslations {
  type Item = Strand<RNA>;

  fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
    if let Some(next) = self.next_at_index(0) {
      Some(Strand::new(next))
    } else {
      None
    }
  }
}

impl RNAReverseTranslations {
  pub fn new(protein: &Strand<AminoAcid>) -> Self {
    let length = protein.nucleotides.len();

    let mut indexes: Vec<usize> = Vec::with_capacity(length);
    let mut possible_translations = Vec::with_capacity(length);

    for i in 0..length {
      indexes.push(0);
      possible_translations.push(RNA::reverse_translations(&protein.nucleotides[i]));
    }

    RNAReverseTranslations {
      indexes: indexes,
      possible_translations: possible_translations,
    }
  }

  fn next_at_index(&mut self, index: usize) -> Option<Vec<RNA>> {
    let i = self.indexes[index];
    if i == self.possible_translations[index].len() {
      return None;
    }

    let current_index_value = self.possible_translations[index][i];
    let mut value = vec![
      current_index_value.0,
      current_index_value.1,
      current_index_value.2,
    ];

    if index == self.possible_translations.len() - 1 {
      self.indexes[index] += 1;
      return Some(value);
    } else if let Some(next_index_value) = self.next_at_index(index + 1) {
      value.extend(next_index_value);
      return Some(value);
    } else {
      self.indexes[index] += 1;
      for ii in index + 1..self.indexes.len() {
        self.indexes[ii] = 0;
      }
      return self.next_at_index(index);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reverse_translation() {
    let iter = RNAReverseTranslations::new(&Strand::new(vec![
      AminoAcid::W,
      AminoAcid::Y,
      AminoAcid::Stop,
    ]));
    let translations: Vec<Strand<RNA>> = iter.collect();
    assert_eq!(translations.len(), 6);
  }
}
