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
        .version("0.1.0") // バージョン情報
        .author("SHIKUMA Naokata <snaokata@gmail.com>") // 作者情報
        .about("Tablec is a table converter tool.") // このアプリについて
        .arg(
            Arg::with_name("FILE") // 位置引数を定義
                .help("input file") // ヘルプメッセージ
                .required(false), // この引数は必須であることを定義
        )
        .arg(
            Arg::with_name("INPUT_FORMAT") // オプションを定義
                .help("input format") // ヘルプメッセージ
                .short("i") // ショートコマンド
                .long("input") // ロングコマンド
                .takes_value(true), // 値を持つことを定義
        );

    let matches = app.get_matches();
    println!("{:?}", matches);
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
