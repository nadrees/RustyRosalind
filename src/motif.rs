use lazy_static::lazy_static;
use regex::{Error, Regex};

pub type MotifError = Error;

/// A motif that can be searched for in a given
pub struct Motif {
    re: Regex,
}

impl Motif {
    pub fn new(pattern: &str) -> Result<Motif, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(?P<elem>[A-Z])\}").unwrap();
        }
        let pattern = RE.replace_all(&pattern, "[^$elem]");
        Ok(Motif {
            re: Regex::new(&pattern)?,
        })
    }

    pub fn matches<'a>(&'a self, haystack: &'a str) -> MotifMatches {
        MotifMatches {
            motif: self,
            position: 0,
            haystack,
        }
    }
}

pub struct MotifMatches<'m, 'a> {
    motif: &'m Motif,
    position: usize,
    haystack: &'a str,
}

impl Iterator for MotifMatches<'_, '_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.haystack.len() {
            return None;
        }
        match self.motif.re.find_at(self.haystack, self.position) {
            None => None,
            Some(m) => {
                let start = m.start();
                self.position = start + 1;
                Some(start)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() -> Result<(), Error> {
        let m = Motif::new("N{P}[ST]{P}")?;
        let strand = "MKNKFKTQEELVNHLKTVGFVFANSEIYNGLANAWDYGPLGVLLKNNLKNLWWKEFVTKQKDVVGLDSAIILNPLVWKASGHLDNFSDPLIDCKNCKARYRADKLIESFDENIHIAENSSNEEFAKVLNDYEISCPTCKQFNWTEIRHFNLMFKTYQGVIEDAKNVVYLRPETAQGIFVNFKNVQRSMRLHLPFGIAQIGKSFRNEITPGNFIFRTREFEQMEIEFFLKEESAYDIFDKYLNQIENWLVSACGLSLNNLRKHEHPKEELSHYSKKTIDFEYNFLHGFSELYGIAYRTNYDLSVHMNLSKKDLTYFDEQTKEKYVPHVIEPSVGVERLLYAILTEATFIEKLENDDERILMDLKYDLAPYKIAVMPLVNKLKDKAEEIYGKILDLNISATFDNSGSIGKRYRRQDAIGTIYCLTIDFDSLDDQQDPSFTIRERNSMAQKRIKLSELPLYLNQKAHEDFQRQCQK";
        let matches: Vec<usize> = m.matches(&strand).collect();
        assert_eq!(matches, vec![84, 117, 141, 305, 394]);
        Ok(())
    }

    mod new_tests {
        use super::*;

        macro_rules! new_test {
      ($($name:ident: $value:expr,)*) => {
        $(
          #[test]
          fn $name() -> Result<(), Error> {
            let (motif_str, expected_regex_str) = $value;
            let m = Motif::new(motif_str)?;
            let re = Regex::new(expected_regex_str)?;
            assert_eq!(m.re.as_str(), re.as_str());
            Ok(())
          }
        )*
      };
    }

        new_test! {
          sanity: ("{P}", "[^P]"),
          no_replacement: ("P", "P"),
          multiple_replacements: ("{P}{A}", "[^P][^A]"),
        }
    }
}
