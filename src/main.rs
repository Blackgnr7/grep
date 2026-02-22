use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("por favor coloque exetamente assim, grep 'file' 'palavra'");
        return;
    }
    let file = File::open(&args[1]).expect("deu erro");
    let read = BufReader::new(file);
    for line in read.lines(){
        let linha = line.unwrap();
        if linha.contains(&args[2]){
            println!("{:?}", linha)
        }
    }
}
