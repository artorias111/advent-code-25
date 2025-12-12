use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(filename).unwrap();
    let buffile = BufReader::new(file);
    Ok(buffile.lines())
}
