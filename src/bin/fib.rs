use clap::{App, Arg};

struct FibRabbit {
    num_offspring_per_generation: u32,
    n0: u128,
    n1: u128,
    generation: u32,
}

impl FibRabbit {
    pub fn new(num_offspring_per_generation: u32) -> FibRabbit {
        FibRabbit {
            num_offspring_per_generation,
            n0: 1,
            n1: 1,
            generation: 0,
        }
    }
}

impl Iterator for FibRabbit {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        if self.generation == 0 || self.generation == 1 {
            self.generation += 1;
            return Some(1u128);
        }

        self.generation += 1;
        let n2 = self.n1 + (self.n0 * (self.num_offspring_per_generation as u128));
        self.n0 = self.n1;
        self.n1 = n2;
        Some(n2)
    }
}

fn main() {
    let matches = App::new("FIB")
        .args(&vec![
            Arg::with_name("n").required(true).index(1),
            Arg::with_name("k").required(true).index(2),
        ])
        .get_matches();

    let n = matches.value_of("n").unwrap().parse::<usize>().unwrap();
    let k = matches.value_of("k").unwrap().parse::<u32>().unwrap();

    let mut fib_rabbits = FibRabbit::new(k);
    let answer = fib_rabbits.nth(n - 1).unwrap();
    print!("{}", answer);
}
