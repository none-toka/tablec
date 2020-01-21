use std::error::Error;
use std::io;
use std::process;

use clap::{App, Arg, SubCommand};

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
            Arg::with_name("pa") // 位置引数を定義
                .help("sample positional argument") // ヘルプメッセージ
                .required(true), // この引数は必須であることを定義
        )
        .arg(
            Arg::with_name("flg") // フラグを定義
                .help("sample flag") // ヘルプメッセージ
                .short("f") // ショートコマンド
                .long("flag"), // ロングコマンド
        )
        .arg(
            Arg::with_name("opt") // オプションを定義
                .help("sample option") // ヘルプメッセージ
                .short("o") // ショートコマンド
                .long("opt") // ロングコマンド
                .takes_value(true), // 値を持つことを定義
        )
        .subcommand(
            SubCommand::with_name("sub") // サブコマンドを定義
                .about("sample subcommand") // このサブコマンドについて
                .arg(
                    Arg::with_name("subflg") // フラグを定義
                        .help("sample flag by sub") // ヘルプメッセージ
                        .short("f") // ショートコマンド
                        .long("flag"), // ロングコマンド
                ),
        );

    let matches = app.get_matches();
    println!("{:?}", matches);
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
