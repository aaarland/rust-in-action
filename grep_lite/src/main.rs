use std::{fs::File, io::prelude::*, io::BufReader};

use clap::{App, Arg};
use regex::Regex; // <1>

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", &line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("1.0")
        .about("Searches for patterns in files")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap(); // <2>
                                           //
    let input = args.value_of("input").unwrap();

    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
