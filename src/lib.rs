#[derive(PartialEq, Eq, Hash)]
pub enum DNA { A, C, G, T }

pub fn parse_nucleotides(input: &str) -> Vec<DNA> {
  input.chars().map(|c| match c {
    'A' => DNA::A,
    'C' => DNA::C,
    'G' => DNA::G,
    'T' => DNA::T,
    x => panic!("Invalid character read: {}", x)
  }).collect()
}
