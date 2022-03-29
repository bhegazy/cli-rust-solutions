use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
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
        .default_value("-"),
    )
    .arg(
        Arg::with_name("number")
            .help("Print line numbers")
            .short("n")
            .long("number")
            .takes_value(false)
            .conflicts_with("number_nonblank"),
    ).arg(
        Arg::with_name("number_nonblank")
        .help("Print line numbers not including blank lines")
        .short("b")
        .long("number-nonblank")
        .takes_value(false),
        )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// This will not compile and will error out " doesn't have a size known at compile-time"
// fn open_without_box(filename: &str) -> MyResult<dyn BufRead> {
//     match filename {
//         "-" => Ok(BufReader::new(io::stdin())),
//         _ => Ok(BufReader::new(File::open(filename))),
//     }
// }

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files{
        match open(&file) {
            Err(err) => eprintln!("Failed to open file {}: {}", file, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}",line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_num += 1;
                            println!("{:>6}\t{}",last_num , line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            },
        }
    }
    Ok(())
}