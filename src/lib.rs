pub mod nucleotides {
    /// The 4 base nucleotides that make up a DNA strand
    pub enum DNA { A, C, G, T }

    impl DNA {
        /// converts a character to its DNA strand, or panics
        pub fn parse(c: char) -> DNA {
            match c {
                'A' => DNA::A,
                'C' => DNA::C,
                'G' => DNA::G,
                'T' => DNA::T,
                x => panic!("Invalid DNA nucleotide: {:?}", x)
            }
        }

        /// converts a string to a DNA string, or panics
        /// if any character is invalid.
        pub fn parse_str(s: &str) -> Vec<DNA> {
            s.trim()
                .chars()
                .map(|c| DNA::parse(c))
                .collect()
        }
    }
}