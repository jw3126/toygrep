extern crate clap;
use std::io::prelude::*;
use clap::{Arg};
use std::io::{BufReader};
use std::fs::{File};
use std::path::{Path};

fn search_file(pattern:&str, filepath:&Path) {
    let file = File::open(filepath)
        .expect(&format!("Cannot open {:?}", filepath));
    let buf = BufReader::new(file);
    for (i,line) in buf.lines().enumerate() {
        let s = line.unwrap();
        if s.contains(pattern) {
            println!("{} {}",i+1,s)
        }
    };
}

fn main() {
    
    let matches = clap::App::new("toygrep")
                          .version("0.1")
                          .author("Jan Weidner <jw3126@gmail.com>")
                          .about("Simple grep clone.")
                          .arg(Arg::with_name("PATTERN")
                               .help("Pattern that should be searched for")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("FILE")
                               .required(true)
                               .index(2)
                               .help("File that should be searched"))
                          .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap();
    let filename = matches.value_of("FILE").unwrap();
    let filepath = Path::new(filename);
    search_file(pattern, filepath);
}
