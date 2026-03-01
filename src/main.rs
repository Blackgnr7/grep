use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[command(name = "NetVoid")]
struct Cli {
    #[arg(long, short)]
    file: Option<String>,

    #[arg(long, short)]
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(texto) = &cli.text {
        if let Some(arquivo) = &cli.file {
            let file = File::open(arquivo).expect("deu erro");
            let read = BufReader::new(file);
            let mut i = 1;
            for line in read.lines() {
                let linha = line.unwrap();
                if linha.contains(texto) {
                    println!("{:?}, {}", linha, i)
                }
                i = i+1;
            }
        }
    }
}
