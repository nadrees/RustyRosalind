use clap::Parser;
use rusty_rosalind::rabbits::Rabbits;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap()]
  n: usize,
  #[clap()]
  k: u8,
}

fn main() {
  let args = Args::parse();
  let mut rabbits = Rabbits::new(args.k);
  println!("{}", rabbits.nth(args.n - 1).unwrap());
}
