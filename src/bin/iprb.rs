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

  let total_pop_size = args.k + args.m + args.n;

  let p_k_selected_first = args.k as f64 / total_pop_size as f64;
  let p_k_selected_second = ((args.m + args.n) as f64 / total_pop_size as f64)
    * (args.k as f64 / (total_pop_size - 1) as f64);

  let p_m_selected_first = args.m as f64 / total_pop_size as f64;
  let p_m_selected_first_and_m_selected_second =
    p_m_selected_first * (args.m - 1) as f64 / (total_pop_size - 1) as f64;
  let p_m_selected_first_and_n_selected_second =
    p_m_selected_first * args.n as f64 / (total_pop_size - 1) as f64;

  let p_n_selected_first_and_m_selected_second =
    (args.n as f64 / total_pop_size as f64) * (args.m as f64 / (total_pop_size - 1) as f64);

  let answer = p_k_selected_first
    + p_k_selected_second
    + (p_m_selected_first_and_n_selected_second * 0.5)
    + (p_n_selected_first_and_m_selected_second * 0.5)
    + (p_m_selected_first_and_m_selected_second * 0.75);
  println!("{}", answer);
}
