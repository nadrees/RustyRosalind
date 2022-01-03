use clap::Parser;
use rusty_rosalind::phenotype::{Phenotype, PhenotypeCombinator};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap()]
  dominant_dominant: usize,
  #[clap()]
  dominant_hetero: usize,
  #[clap()]
  dominant_recessive: usize,
  #[clap()]
  hetero_hetero: usize,
  #[clap()]
  hetero_recessive: usize,
  #[clap()]
  recessive_recessive: usize,
}

const NUM_OFFSPRING: usize = 2;

fn main() {
  let args = Args::parse();
  let populations = vec![
    args.dominant_dominant,
    args.dominant_hetero,
    args.dominant_recessive,
    args.hetero_hetero,
    args.hetero_recessive,
    args.recessive_recessive,
  ];
  let events = vec![
    // AA-AA
    PhenotypeCombinator::new(Phenotype::HomozygousDominant, Phenotype::HomozygousDominant),
    // AA-Aa
    PhenotypeCombinator::new(Phenotype::HomozygousDominant, Phenotype::Heterozygous),
    // AA-aa
    PhenotypeCombinator::new(
      Phenotype::HomozygousDominant,
      Phenotype::HomozygousRecessive,
    ),
    // Aa-Aa
    PhenotypeCombinator::new(Phenotype::Heterozygous, Phenotype::Heterozygous),
    // Aa-aa
    PhenotypeCombinator::new(Phenotype::Heterozygous, Phenotype::HomozygousRecessive),
    // aa-aa
    PhenotypeCombinator::new(
      Phenotype::HomozygousRecessive,
      Phenotype::HomozygousRecessive,
    ),
  ];
  let result: f32 = populations
    .iter()
    .zip(events.iter())
    .map(|(p, e)| *p as f32 * e.probability_dominant())
    .sum();
  println!("{}", result * NUM_OFFSPRING as f32);
}
