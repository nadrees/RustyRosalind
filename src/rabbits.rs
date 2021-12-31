pub struct Rabbits {
  offspring_per_step: u8,
  children: u128,
  pub population: u128,
}

impl Rabbits {
  /// creates a new Rabbits where each pair of breeding rabbits
  /// produces the specified number of offspring pairs each step
  pub fn new(offspring: u8) -> Self {
    Rabbits {
      offspring_per_step: offspring,
      children: 1,
      population: 0,
    }
  }
}

impl Iterator for Rabbits {
  type Item = u128;

  fn next(&mut self) -> Option<Self::Item> {
    let offpsring_this_step = self.population * self.offspring_per_step as u128;
    self.population += self.children;
    self.children = offpsring_this_step;
    Some(self.population)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rabbits() {
    let mut rabbits = Rabbits::new(3);
    assert_eq!(rabbits.nth(4), Some(19));
  }
}
