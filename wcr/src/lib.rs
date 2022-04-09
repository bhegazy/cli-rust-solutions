use clap::{Command, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.0.1")
        .author("Bill Hegazy <bill.hegazy@gmail.com>")
        .about("Rust wc")
        // .arg_required_else_help(true)
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple_occurrences(true)
                .allow_invalid_utf8(true)
                .default_value("-")
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Show line count")
                .short('l')
                .long("lines")
                .takes_value(false)
        )
        .arg(
            Arg::new("words")
                .value_name("WORDS")
                .help("Show words count")
                .short('w')
                .long("words")
                .takes_value(false)
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .short('c')
                .long("bytes")
                .takes_value(false)
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .help("Show character count")
                .short('m')
                .long("chars")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .get_matches();
    let (mut lines,
        mut words,
        mut bytes,
        chars,
    ) = (
        matches.is_present("lines"),
        matches.is_present("words"),
        matches.is_present("bytes"),
        matches.is_present("chars"),
    );
    if !lines && !words && !bytes && !chars {
        lines = true;
        words = true;
        bytes = true;
    }
    // if [lines, words, bytes, chars].iter().all(|v| !v) {
    //     (lines, words, bytes) = (true, true, true);
    // }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = Vec::new();
    loop {
        match file.read_until(b'\n', &mut line) {
            Ok(0) => break,
            Ok(_) => {
                num_lines += 1;
                num_bytes += line.len();
                // println!("len: {}, {:?} ", num_bytes, String::from_utf8_lossy(&line));
                String::from_utf8_lossy(&line).split_whitespace().for_each(|_|num_words += 1);
                String::from_utf8_lossy(&line).chars().for_each(|_| num_chars += 1);
                line.clear();
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let file_num = config.files.len();
    let mut total_bytes = 0;
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    for (n, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let data = count(file)?;
                total_bytes += data.num_bytes;
                total_lines += data.num_lines;
                total_words += data.num_words;
                total_chars += data.num_chars;

                if filename == "-" {
                    println!("{:>8}{:>8}{:>8}", data.num_lines, data.num_words, data.num_bytes);
                    break
                }
                if config.bytes && config.lines && config.words {
                    println!("{:>8}{:>8}{:>8} {}", data.num_lines, data.num_words, data.num_bytes, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8}{:>8}{:>8} total", total_lines, total_words, total_bytes);
                    }
                } else if config.lines && config.bytes {
                    println!("{:>8}{:>8} {}", data.num_lines, data.num_bytes, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8}{:>8} total", total_lines, total_bytes);
                    }
                } else if config.lines && config.words {
                    println!("{:>8}{:>8} {}", data.num_lines, data.num_words, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8}{:>8} total", total_lines, total_words);
                    }
                } else if config.words && config.bytes {
                    println!("{:>8}{:>8} {}", data.num_words, data.num_bytes, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8}{:>8} total", total_words, total_bytes);
                    }
                } else if config.lines {
                    println!("{:>8} {}", data.num_lines, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8} total", total_lines);
                    }
                } else if config.bytes {
                    println!("{:>8} {}", data.num_bytes, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8} total", total_bytes);
                    }
                } else if config.words {
                    println!("{:>8} {}", data.num_words, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8} total", total_words);
                    }
                } else if config.chars {
                    println!("{:>8} {}", data.num_chars, filename);
                    if file_num > 1 && n+1 == file_num {
                        println!("{:>8} total", total_chars);
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{count, FileInfo};

    #[test]
    fn test_count() {
        let text = "Be careful for what you wish for, cause you just might get it all.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 14,
            num_bytes: 68,
            num_chars: 68,
        };
        assert_eq!(info.unwrap(), expected);
    }
}