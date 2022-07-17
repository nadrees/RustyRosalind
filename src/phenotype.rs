pub enum Phenotype {
    Heterozygous,
    HomozygousDominant,
    HomozygousRecessive,
}

/// Calculates probability of particular outcomes
pub struct PhenotypeCombinator {
    p1: Phenotype,
    p2: Phenotype,
}

impl PhenotypeCombinator {
    pub fn new(p1: Phenotype, p2: Phenotype) -> Self {
        PhenotypeCombinator { p1: p1, p2: p2 }
    }

    /// returns the probability (0.0 - 1.0) that the resulting combination would
    /// contain the dominant phenotype (i.e. Aa or AA)
    pub fn probability_dominant(&self) -> f32 {
        match (&self.p1, &self.p2) {
            (Phenotype::HomozygousDominant, _) | (_, Phenotype::HomozygousDominant) => 1.0,
            (Phenotype::Heterozygous, Phenotype::Heterozygous) => 0.75,
            (Phenotype::Heterozygous, Phenotype::HomozygousRecessive)
            | (Phenotype::HomozygousRecessive, Phenotype::Heterozygous) => 0.5,
            (Phenotype::HomozygousRecessive, Phenotype::HomozygousRecessive) => 0.0,
        }
    }

    /// returns the probability (0.0 - 1.0) that the resulting combination would
    /// contain the recesive phenotype (i.e. aa)
    pub fn probability_recessive(&self) -> f32 {
        1.0 - self.probability_dominant()
    }
}
