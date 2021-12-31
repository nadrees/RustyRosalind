use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap()]
  k: u32,
  #[clap()]
  m: u32,
  #[clap()]
  n: u32,
}

fn main() {
  let args = Args::parse();

  let totalPopSize = args.k + args.m + args.n;

  let p_kSelectedFirst = args.k as f64 / totalPopSize as f64;
  let p_kSelectedSecond =
    ((args.m + args.n) as f64 / totalPopSize as f64) * (args.k as f64 / (totalPopSize - 1) as f64);

  let p_mSelectedFirst = args.m as f64 / totalPopSize as f64;
  let p_mSelectedFirst_And_mSelectedSecond =
    p_mSelectedFirst * (args.m - 1) as f64 / (totalPopSize - 1) as f64;
  let p_mSelectedFirst_And_nSelectedSecond =
    p_mSelectedFirst * args.n as f64 / (totalPopSize - 1) as f64;

  let p_nSelectedFirst_And_mSelectedSecond =
    (args.n as f64 / totalPopSize as f64) * (args.m as f64 / (totalPopSize - 1) as f64);

  let answer = p_kSelectedFirst
    + p_kSelectedSecond
    + (p_mSelectedFirst_And_nSelectedSecond * 0.5)
    + (p_nSelectedFirst_And_mSelectedSecond * 0.5)
    + (p_mSelectedFirst_And_mSelectedSecond * 0.75);
  println!("{}", answer);
}
