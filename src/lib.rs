pub mod fasta;
pub mod nucleotides;

#[macro_export]
macro_rules! start_standard_program {
    ($program_name:tt) => {{
        let matches = clap::App::new($program_name)
            .arg(clap::Arg::with_name("input_file").required(true).index(1))
            .get_matches();
        let f_name = matches.value_of("input_file").unwrap();
        std::fs::read_to_string(f_name).unwrap()
    }};
}
