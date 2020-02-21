use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

use anyhow::{Context, Result};

use clap::{App, Arg};

use serde::Deserialize;

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
    conv: Box<dyn Fn(csv::StringRecord) -> Vec<csv::StringRecord>>,
) -> Result<()> {
    let headers = rdr.headers()?;
    wtr.write_record(headers)?;
    for result in rdr.records() {
        let record = result?;
        for converted in conv(record) {
            wtr.write_record(converted.iter())?;
        }
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(tag = "command")]
enum ConvertCommand {
    HSplit { col: String, sep: String },
}

fn converter_identity() -> Box<dyn Fn(csv::StringRecord) -> Vec<csv::StringRecord> + 'static> {
    Box::new(|x| vec![x])
}

fn converter_hsplit(
    col: String,
    sep: String,
) -> Result<Box<dyn Fn(csv::StringRecord) -> Vec<csv::StringRecord>>> {
    // TODO error message
    let col_num_tmp: usize = col.parse()?;
    let col_num = col_num_tmp - 1;
    Ok(Box::new(move |rec| {
        let field = rec.get(col_num);
        if field == None {
            return vec![rec];
        }
        let fields = field.unwrap().split(&sep);
        let mut ret = Vec::new();
        for v in fields {
            let mut r = csv::StringRecord::new();
            for (i, f) in rec.iter().enumerate() {
                r.push_field(if i == col_num { v } else { f });
            }
            ret.push(r);
        }
        ret
    }))
}

fn converter(command: String) -> Result<Box<dyn Fn(csv::StringRecord) -> Vec<csv::StringRecord>>> {
    let cmd = serde_json::from_str(&command)?;
    match cmd {
        ConvertCommand::HSplit { col, sep } => converter_hsplit(col, sep),
    }
}

fn execute(params: ArgParameters) -> Result<()> {
    // Build the CSV reader and iterate over each record.
    let r = reader(&params.input_file)?;
    let mut rdr = table_reader(&params.input_format).from_reader(r);
    let c = match params.convert_command {
        Some(cmd) => converter(cmd)?,
        None => converter_identity(),
    };
    let w = writer(&params.output_file)?;
    let mut wtr = table_writer(&params.output_format).from_writer(w);
    write(&mut rdr, &mut wtr, c)
}

#[derive(Debug)]
struct ArgParameters {
    input_file: Option<String>,
    input_format: String,
    output_file: Option<String>,
    output_format: String,
    convert_command: Option<String>,
}

const INPUT_FILE: &str = "FILE";
const INPUT_FORMAT: &str = "INPUT_FORMAT";
const OUTPUT_FILE: &str = "OUTPUT_FILE";
const OUTPUT_FORMAT: &str = "OUTPUT_FORMAT";
const CONVERT_COMMAND: &str = "CONVERT_COMMAND";

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
        )
        .arg(
            Arg::with_name(CONVERT_COMMAND)
                .help("convert command (TBD)")
                .short("c")
                .long("convert")
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
        convert_command: matched.value_of(CONVERT_COMMAND).map(|s| s.to_string()),
    }
}

fn main() {
    let arg = parse();
    if let Err(err) = execute(arg) {
        println!("error running execute: {}", err);
        process::exit(1);
    }
}
