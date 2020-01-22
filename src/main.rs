use std::error::Error;
use std::io;
use std::process;

use clap::{App, Arg};

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[derive(Debug)]
struct ArgParameters {
    input_file: Option<String>,
    input_format: String,
    output_file: Option<String>,
    output_format: String,
}

const INPUT_FILE: &str = "FILE";
const INPUT_FORMAT: &str = "INPUT_FORMAT";
const OUTPUT_FILE: &str = "OUTPUT_FILE";
const OUTPUT_FORMAT: &str = "OUTPUT_FORMAT";

fn parse() -> ArgParameters {
    let app = App::new("tablec")
        .version("0.1.0")
        .author("SHIKUMA Naokata <snaokata@gmail.com>")
        .about("Tablec is a table converter tool.")
        .arg(
            Arg::with_name(INPUT_FILE)
                .help("input file")
                .required(false),
        )
        .arg(
            Arg::with_name(INPUT_FORMAT)
                .help("input format")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(OUTPUT_FORMAT)
                .help("output format")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(OUTPUT_FILE)
                .help("output file")
                .short("f")
                .long("file")
                .takes_value(true),
        );

    let matched = app.get_matches();
    ArgParameters {
        input_file: matched.value_of(INPUT_FILE).map(|s| s.to_string()),
        input_format: matched
            .value_of(INPUT_FORMAT)
            .map(|s| s.to_string())
            .expect(&format!("Not given: {}", INPUT_FORMAT)),
        output_file: matched.value_of(OUTPUT_FILE).map(|s| s.to_string()),
        output_format: matched
            .value_of(OUTPUT_FORMAT)
            .map(|s| s.to_string())
            .expect(&format!("Not given: {}", OUTPUT_FORMAT)),
    }
}
fn main() {
    let matches = parse();
    println!("{:?}", matches);
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
