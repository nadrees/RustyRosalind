pub struct Rabbits {
  offspring_per_step: u8,
  cohorts: Vec<RabbitCohort>,
  max_age: Option<u8>,
}

impl Rabbits {
  /// creates a new Rabbits where each pair of breeding rabbits
  /// produces the specified number of offspring pairs each step
  pub fn new_with_offsping(offspring: u8) -> Self {
    Rabbits::new(Some(offspring), None)
  }

  pub fn new_with_max_age(max_age: u8) -> Self {
    Rabbits::new(None, Some(max_age))
  }

  pub fn new(offspring: Option<u8>, max_age: Option<u8>) -> Self {
    let mut rabbits = Rabbits {
      offspring_per_step: offspring.unwrap_or(1),
      cohorts: vec![],
      max_age: max_age,
    };
    rabbits.add_cohort(1);
    rabbits
  }

  fn add_cohort(&mut self, population: u128) -> () {
    self.cohorts.push(RabbitCohort {
      age: 1,
      population: population,
      max_age: self.max_age,
    });
  }

  fn get_adult_population(&self) -> u128 {
    self
      .cohorts
      .iter()
      // adults are at least 2 iterations old
      .filter(|c| c.age > 1)
      .map(|c| c.population)
      .sum()
  }

  fn get_total_population(&self) -> u128 {
    self.cohorts.iter().map(|c| c.population).sum()
  }
}

impl Iterator for Rabbits {
  type Item = u128;

  fn next(&mut self) -> Option<Self::Item> {
    let total_pop = self.get_total_population();
    let adult_population_this_step = self.get_adult_population();
    self.cohorts.iter_mut().for_each(|c| c.age += 1);
    if self.max_age != None {
      self.cohorts.retain(|c| !c.is_dead());
    }
    if adult_population_this_step > 0 {
      self.add_cohort(adult_population_this_step * self.offspring_per_step as u128);
    }
    Some(total_pop)
  }
}

/// Represents a cohort of rabbits that are the same
/// age.
#[derive(Copy, Clone)]
struct RabbitCohort {
  /// the current age of the cohort
  age: u8,
  /// the number of pairs of rabbits in the cohort
  population: u128,
  /// the maximum age of the rabbits in this cohort.
  /// when age > max_age, these rabbits will die and
  /// no longer produce offspring or count in the total
  /// population
  max_age: Option<u8>,
}

impl RabbitCohort {
  fn is_dead(&self) -> bool {
    match self.max_age {
      Some(max) if self.age > max => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rabbits_with_offspring() {
    let mut rabbits = Rabbits::new_with_offsping(3);
    assert_eq!(rabbits.nth(4), Some(19));
  }

  #[test]
  fn test_rabbits_with_max_age() {
    let mut rabbits = Rabbits::new_with_max_age(3);
    assert_eq!(rabbits.nth(5), Some(4));
  }
}
