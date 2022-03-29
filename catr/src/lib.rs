use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
    .version("0.1.0")
    .author("Bill hegazy <bill.hegazy@gmail.com>")
    .about("Rust cat")
    .arg(
        Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value(""),
    )
    .arg(
        Arg::with_name("number_lines")
        .help("Print line numbers")
        .short("n")
        .long("number")
        .takes_value(false),
    ).arg(
        Arg::with_name("number_nonblank_lines")
        .help("Print line numbers not including blank lines")
        .short("b")
        .long("number-nonblank")
        .takes_value(false),
        )
    .get_matches();

    Ok(Config {
        files: matches.value_of_lossy("files").unwrap(),
        number_lines: matches.value_of(name),
        number_nonblank_lines: ,
    })
}



pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}