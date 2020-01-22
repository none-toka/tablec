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

fn main() {
    let app = App::new("tablec")
        .version("0.1.0")
        .author("SHIKUMA Naokata <snaokata@gmail.com>")
        .about("Tablec is a table converter tool.")
        .arg(Arg::with_name("FILE").help("input file").required(false))
        .arg(
            Arg::with_name("INPUT_FORMAT")
                .help("input format")
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT_FORMAT")
                .help("output format")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT_FILE")
                .help("output file")
                .short("f")
                .long("file")
                .takes_value(true),
        );

    let matches = app.get_matches();
    println!("{:?}", matches);
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
