use std::io::prelude::*;
use clap::{Arg, ArgMatches};
use std::io::{BufReader};
use std::fs::{File};
use std::path::{Path,PathBuf};


pub struct SearchTask {
    pattern:String,
    filepath:PathBuf,
    case_sensitive:bool,
}

impl SearchTask {

    pub fn run(&self) {
        let &SearchTask {ref pattern, 
            ref filepath, 
            case_sensitive} = self;

        let pattern2 = if case_sensitive {
            pattern.clone()
        } else {
            pattern.to_lowercase()
        };
        let pat = pattern2.as_str();

        let file = File::open(&filepath)
            .expect(&format!("Cannot open {:?}", filepath));
        let buf = BufReader::new(file);
        for (i,line) in buf.lines().enumerate() {
            let s = line.unwrap();
            let s_normalized = if case_sensitive {
                s.clone()
            } else {
                s.clone().to_lowercase()
            };
            if s_normalized.contains(pat) {
                println!("{} {}",i+1,s)
            }
        };
    }
}

pub fn parse_search_task(matches:ArgMatches) -> SearchTask {
    let pattern = matches.value_of("PATTERN").unwrap().to_string();
    let filename = matches.value_of("FILE").unwrap();
    let filepath = Path::new(filename).canonicalize()
        .expect(&format!("Problem with path {}", filename));
    let case_sensitive = matches.occurrences_of("CASE_INSENSITIVE") == 0;
    let task = SearchTask {pattern, filepath, case_sensitive};
    task
}

pub fn create_app() -> clap::App<'static, 'static> {
    clap::App::new("toygrep")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Simple grep clone.")
            .arg(Arg::with_name("PATTERN")
                 .help("Pattern that should be searched for")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("FILE")
                 .required(true)
                 .index(2)
                 .help("File that should be searched"))
            .arg(Arg::with_name("CASE_INSENSITIVE")
                 .short("i")
                 .long("case-insensitive")
                 .help("Treat all letters as lowercase"))
}
