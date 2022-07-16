use clap::Parser;
use futures::future::join_all;
use lazy_static::lazy_static;
use rusty_rosalind::args::FileArgs as Args;
use rusty_rosalind::fasta::Fasta;
use rusty_rosalind::motif::Motif;
use rusty_rosalind::nucleotides::protein::AminoAcid;
use std::error::Error;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let lines = args.read_file().unwrap();
    let handles = lines
        .into_iter()
        .map(|line| tokio::spawn(async move { process_id(line.unwrap()).await }));
    let results = join_all(handles).await;
    for result in results {
        if let Ok(r) = result.unwrap() {
            let (name, matches) = r;
            if matches.len() > 0 {
                println!("{}", name);
                println!("{:?}", matches);
            }
        }
    }
}

async fn process_id(id: String) -> Result<(String, Vec<usize>), Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(format!("http://www.uniprot.org/uniprot/{}.fasta", id))
        .await?
        .text()
        .await?;
    println!("{}", response);
    let record: Fasta<AminoAcid> =
        Fasta::from(response.split("\n").filter(|l| l.chars().count() > 0));
    lazy_static! {
        static ref M: Motif = Motif::new("N{P}[ST]{P}").unwrap();
    }
    let matches: Vec<usize> = M
        .matches(&format!("{}", record.strand))
        .map(|m| m + 1)
        .collect();
    Ok((id, matches))
}
