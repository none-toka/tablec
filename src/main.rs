use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

use anyhow::{Context, Result};

use clap::{App, Arg};

fn reader(file: &Option<String>) -> Result<Box<dyn Read>> {
    match file {
        Some(f) => {
            let r = File::open(&f).with_context(|| format!("Cannot open file to read: {}", &f))?;
            Ok(Box::new(r))
        }
        None => Ok(Box::new(io::stdin())),
    }
}

fn writer(file: &Option<String>) -> Result<Box<dyn Write>> {
    match file {
        Some(f) => {
            let r =
                File::create(&f).with_context(|| format!("Cannot open file to write: {}", &f))?;
            Ok(Box::new(r))
        }
        None => Ok(Box::new(io::stdout())),
    }
}

const TSV_FORMAT: &str = "tsv";
const TSV_DELIMITER: u8 = b'\t';
const CSV_DELIMITER: u8 = b',';

fn table_reader(input_format: &str) -> csv::ReaderBuilder {
    let mut ret = csv::ReaderBuilder::new();
    match input_format {
        TSV_FORMAT => ret.delimiter(TSV_DELIMITER),
        _ => ret.delimiter(CSV_DELIMITER),
    };
    ret
}

fn table_writer(output_format: &str) -> csv::WriterBuilder {
    let mut ret = csv::WriterBuilder::new();
    match output_format {
        TSV_FORMAT => ret.delimiter(TSV_DELIMITER),
        _ => ret.delimiter(CSV_DELIMITER),
    };
    ret
}

fn write(
    rdr: &mut csv::Reader<std::boxed::Box<(dyn std::io::Read)>>,
    wtr: &mut csv::Writer<std::boxed::Box<(dyn std::io::Write)>>,
) -> Result<()> {
    let headers = rdr.headers()?;
    wtr.write_record(headers)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        wtr.write_record(record.iter())?;
    }
    Ok(())
}

fn execute(params: &ArgParameters) -> Result<()> {
    // Build the CSV reader and iterate over each record.
    let r = reader(&params.input_file)?;
    let mut rdr = table_reader(&params.input_format).from_reader(r);
    let w = writer(&params.output_file)?;
    let mut wtr = table_writer(&params.output_format).from_writer(w);
    write(&mut rdr, &mut wtr)
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
    if let Err(err) = execute(&parse()) {
        println!("error running execute: {}", err);
        process::exit(1);
    }
}
