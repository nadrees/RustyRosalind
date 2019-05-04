use rusty_rosalind::fasta::Record;
use rusty_rosalind::start_standard_program;

fn main() {
    let file_contents = start_standard_program!("GC");
    let records = Record::parse_records(&file_contents).unwrap();
    let answer = records
        .iter()
        .max_by(|record1, record2| {
            let record1gc = record1.dna_string.gc_content();
            let record2gc = record2.dna_string.gc_content();
            record1gc.partial_cmp(&record2gc).expect("Got NaN")
        })
        .unwrap();
    println!("{}", answer.name);
    println!("{}", answer.dna_string.gc_content() * 100.0f32);
}
